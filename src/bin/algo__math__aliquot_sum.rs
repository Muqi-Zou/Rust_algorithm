#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::aliquot_sum as source;

fn main() {
    println!("math/aliquot_sum.rs");
}
