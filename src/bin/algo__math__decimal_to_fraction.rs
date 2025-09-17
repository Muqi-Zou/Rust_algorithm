#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::decimal_to_fraction as source;

fn main() {
    println!("math/decimal_to_fraction.rs");
}
