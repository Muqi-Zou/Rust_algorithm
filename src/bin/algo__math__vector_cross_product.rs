#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::vector_cross_product as source;

fn main() {
    println!("math/vector_cross_product.rs");
}
