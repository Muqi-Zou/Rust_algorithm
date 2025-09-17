#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::karatsuba_multiplication as source;

fn main() {
    println!("math/karatsuba_multiplication.rs");
}
