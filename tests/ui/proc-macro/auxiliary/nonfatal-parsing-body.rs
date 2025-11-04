use proc_macro::*;
use std::panic::catch_unwind;
use std::str::FromStr;

fn stream(s: &str) {
    println!("{:?}", TokenStream::from_str(s));
}

fn lit(s: &str) {
    println!("{:?}", Literal::from_str(s));
}

fn stream_catch_unwind(s: &str) {
    if catch_unwind(|| println!("{:?}", TokenStream::from_str(s))).is_ok() {
        eprintln!("{s} did not panic");
    }
}

fn lit_catch_unwind(s: &str) {
    if catch_unwind(|| println!("{:?}", Literal::from_str(s))).is_ok() {
        eprintln!("{s} did not panic");
    }
}

pub fn run() {
    // returns Ok(valid instance)
    lit("123");
    lit("\"ab\"");
    lit("\'b\'");
    lit("'b'");
    lit("b\"b\"");
    lit("c\"b\"");
    lit("cr\"b\"");
    lit("b'b'");
    lit("256u8");
    lit("-256u8");
    stream("-256u8");
    lit("0b11111000000001111i16");
    lit("0xf32");
    lit("0b0f32");
    lit("2E4");
    lit("2.2E-4f64");
    lit("18u8E");
    lit("18.0u8E");
    lit("cr#\"// /* // \n */\"#");
    lit("'\\''");
    lit("'\\\''");
    lit(&format!("r{0}\"a\"{0}", "#".repeat(255)));
    stream("fn main() { println!(\"Hello, world!\") }");
    stream("18.u8E");
    stream("18.0f32");
    stream("18.0f34");
    stream("18.bu8");
    stream("3//\n4");
    stream(
        "\'c\'/*\n
    */",
    );
    stream("/*a*/ //");

    println!("### ERRORS");

    // returns Err(LexError)
    lit("\'c\'/**/");
    lit(" 0");
    lit("0 ");
    lit("0//");
    lit("3//\n4");
    lit("18.u8E");
    lit("/*a*/ //");
    // FIXME: all of the cases below should return an Err and emit no diagnostics, but don't yet.

    // emits diagnostics and returns LexError
    lit("r'r'");
    lit("c'r'");

    // emits diagnostic and returns a seemingly valid tokenstream
    stream("r'r'");
    stream("c'r'");

    for (parse, parse_catch_unwind) in [
        (stream as fn(&str), stream_catch_unwind as fn(&str)),
        (lit, lit_catch_unwind),
    ] {
        // emits diagnostic(s), then panics
        parse_catch_unwind("1 ) 2");
        parse_catch_unwind("( x  [ ) ]");
        parse_catch_unwind("r#");

        // emits diagnostic(s), then returns Ok(Literal { kind: ErrWithGuar, .. })
        parse("0b2");
        parse("0bf32");
        parse("0b0.0f32");
        parse("'\''");
        parse(
            "'
'",
        );
        parse_catch_unwind(&format!("r{0}\"a\"{0}", "#".repeat(256)));

        // emits diagnostic, then, when parsing as a lit, returns LexError, otherwise ErrWithGuar
        parse("/*a*/ 0b2 //");
    }
}
