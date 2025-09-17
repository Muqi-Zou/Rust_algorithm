#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::backtracking::parentheses_generator as source;

fn main() {
    println!("backtracking/parentheses_generator.rs");
}
