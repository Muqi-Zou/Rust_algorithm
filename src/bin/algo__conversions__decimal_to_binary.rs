#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::conversions::decimal_to_binary as source;

fn main() {
    println!("conversions/decimal_to_binary.rs");
}
