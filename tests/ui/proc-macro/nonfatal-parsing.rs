//@ proc-macro: nonfatal-parsing.rs
//@ needs-unwind
//@ edition:2024
//@ dont-require-annotations: ERROR
// FIXME: should be a check-pass test

extern crate proc_macro;
extern crate nonfatal_parsing;

#[path = "auxiliary/nonfatal-parsing-body.rs"]
mod body;

fn main() {
    nonfatal_parsing::run!();
    // FIXME: enable this once the standalone backend exists
    // body::run();
}
