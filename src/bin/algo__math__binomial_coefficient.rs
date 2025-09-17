#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::binomial_coefficient as source;

fn main() {
    println!("math/binomial_coefficient.rs");
}
