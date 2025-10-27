use proc_macro::*;
use std::str::FromStr;
use std::panic::catch_unwind;

fn parse(s: &str) {
    println!("{:?}", TokenStream::from_str(s));
}

fn parse_catch_unwind(s: &str) {
    catch_unwind(|| println!("{:?}", TokenStream::from_str(s)));
}

pub fn run() {
    // works
    parse("123");
    parse("\"ab\"");
    parse("\'b\'");
    parse("'b'");
    parse("b'b'");
    parse("c'b'");
    parse("cr'b'");
    parse("256u8");
    parse("-256u8");
    parse("0b11111000000001111i16");
    parse("0xf32");
    parse("0b0f32");
    parse("fn main() { println!(\"Hello, world!\") }");

    // fails with error
    parse("1 ) 2");
    parse("( x  [ ) ]");

    // fails with compile error
    parse_catch_unwind("0b2");
    parse_catch_unwind("r#");
    parse_catch_unwind("0bf32");
    parse_catch_unwind("0b0.0f32");
}
