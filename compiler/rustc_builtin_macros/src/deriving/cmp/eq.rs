use rustc_ast::{self as ast, MetaItem, Safety};
use rustc_data_structures::fx::FxHashSet;
use rustc_expand::base::{Annotatable, ExtCtxt};
use rustc_span::{Ident, Span, kw, sym};
use thin_vec::{ThinVec, thin_vec};

use crate::deriving::generic::ty::*;
use crate::deriving::generic::*;
use crate::deriving::path_std;

pub(crate) fn expand_deriving_eq(
    cx: &ExtCtxt<'_>,
    span: Span,
    mitem: &MetaItem,
    item: &Annotatable,
    push: &mut dyn FnMut(Annotatable),
    is_const: bool,
) {
    let span = cx.with_def_site_ctxt(span);
    let mut new_stmts = ThinVec::new();

    let trait_def = TraitDef {
        span,
        path: path_std!(cmp::Eq),
        skip_path_as_bound: false,
        needs_copy_as_bound_if_packed: true,
        additional_bounds: Vec::new(),
        supports_unions: true,
        methods: vec![MethodDef {
            name: sym::assert_receiver_is_total_eq,
            generics: Bounds::empty(),
            explicit_self: true,
            nonself_args: vec![],
            ret_ty: Unit,
            attributes: thin_vec![
                cx.attr_word(sym::inline, span),
                cx.attr_nested_word(sym::doc, sym::hidden, span),
                cx.attr_nested_word(sym::coverage, sym::off, span)
            ],
            fieldless_variants_strategy: FieldlessVariantsStrategy::Unify,
            combine_substructure: combine_substructure(Box::new(|a, b, c| {
                cs_total_eq_assert(a, b, c, &mut new_stmts)
            })),
        }],
        associated_types: Vec::new(),
        is_const,
        is_staged_api_crate: cx.ecfg.features.staged_api(),
        safety: Safety::Default,
        document: true,
    };
    trait_def.expand_ext(cx, mitem, item, push, true);
    let bound = cx.trait_bound(
        cx.path_global(
            span,
            vec![
                Ident::new(sym::core, span),
                Ident::new(sym::cmp, span),
                Ident::new(sym::Eq, span),
            ],
        ),
        is_const,
    );
    let body = Some(cx.block(span, new_stmts));
    let mut generics =
        item.clone().expect_item().opt_generics().cloned().unwrap_or(ast::Generics::default());
    generics.params = generics
        .params
        .into_iter()
        .map(|mut param| {
            if matches!(param.kind, ast::GenericParamKind::Type { .. }) {
                param.bounds.push(bound.clone());
                param.kind = ast::GenericParamKind::Type { default: None }
            }
            param
        })
        .collect();
    let mut header = ast::FnHeader::default();
    header.constness = if is_const { ast::Const::Yes(span) } else { ast::Const::No };
    let kind = ast::ItemKind::Fn(Box::new(ast::Fn {
        defaultness: ast::Defaultness::Final,
        sig: ast::FnSig {
            header,
            decl: cx.fn_decl(ThinVec::new(), ast::FnRetTy::Default(span)),
            span,
        },
        ident: Ident::new(sym::assert_receiver_is_total_eq, span),
        generics,
        contract: None,
        body,
        define_opaque: None,
        eii_impls: ThinVec::new(),
    }));
    let item = cx.item(
        span,
        thin_vec![
            cx.attr_nested_word(sym::doc, sym::hidden, span),
            cx.attr_nested_word(sym::coverage, sym::off, span)
        ],
        kind,
    );
    let stmt = cx.stmt_item(span, item);
    let block = ast::ConstItemRhs::Body(cx.expr_block(cx.block(span, thin_vec![stmt])));

    let anon_constant = cx.item_const(
        span,
        Ident::new(kw::Underscore, span),
        cx.ty(span, ast::TyKind::Tup(ThinVec::new())),
        block,
    );
    push(Annotatable::Item(anon_constant));
}

fn cs_total_eq_assert(
    cx: &ExtCtxt<'_>,
    trait_span: Span,
    substr: &Substructure<'_>,
    new_stmts: &mut ThinVec<ast::Stmt>,
) -> BlockOrExpr {
    let mut stmts = ThinVec::new();
    let mut seen_type_names = FxHashSet::default();
    let mut process_variant = |variant: &ast::VariantData| {
        for field in variant.fields() {
            // This basic redundancy checking only prevents duplication of
            // assertions like `AssertParamIsEq<Foo>` where the type is a
            // simple name. That's enough to get a lot of cases, though.
            if let Some(name) = field.ty.kind.is_simple_path()
                && !seen_type_names.insert(name)
            {
                // Already produced an assertion for this type.
            } else {
                // let _: AssertParamIsEq<FieldTy>;
                /*super::assert_ty_bounds(
                    cx,
                    &mut stmts,
                    field.ty.clone(),
                    field.span,
                    &[sym::cmp, sym::AssertParamIsEq],
                );*/
                super::assert_ty_bounds(
                    cx,
                    new_stmts,
                    field.ty.clone(),
                    field.span,
                    &[sym::cmp, sym::AssertParamIsEq],
                );
            }
        }
    };

    match *substr.fields {
        StaticStruct(vdata, ..) => {
            process_variant(vdata);
        }
        StaticEnum(enum_def, ..) => {
            for variant in &enum_def.variants {
                process_variant(&variant.data);
            }
        }
        _ => cx.dcx().span_bug(trait_span, "unexpected substructure in `derive(Eq)`"),
    }
    BlockOrExpr::new_stmts(stmts)
}
