#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::extended_euclidean_algorithm as source;

fn main() {
    println!("math/extended_euclidean_algorithm.rs");
}
