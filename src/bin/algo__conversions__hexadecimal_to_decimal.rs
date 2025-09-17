#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::conversions::hexadecimal_to_decimal as source;

fn main() {
    println!("conversions/hexadecimal_to_decimal.rs");
}
