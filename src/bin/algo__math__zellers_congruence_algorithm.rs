#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::zellers_congruence_algorithm as source;

fn main() {
    println!("math/zellers_congruence_algorithm.rs");
}
