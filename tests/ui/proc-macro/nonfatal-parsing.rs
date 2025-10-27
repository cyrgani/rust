//@ proc-macro: nonfatal-parsing.rs
//@ needs-unwind

extern crate nonfatal_parsing;
extern crate proc_macro;

#[path = "auxiliary/nonfatal-parsing-body.rs"]
mod body;

fn main() {
    nonfatal_parsing::run!();
    //~^ ERROR: invalid digit for a base 2 literal
    //~| ERROR: found invalid character; only `#` is allowed in raw string delimitation: \u{0}
    //~| ERROR: no valid digits found for number [E0768]
    //~| ERROR: binary float literal is not supported


    // FIXME: enable this once the standalone backend exists
    // body::run();
}
