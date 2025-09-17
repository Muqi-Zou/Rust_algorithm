#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::pascal_triangle as source;

fn main() {
    println!("math/pascal_triangle.rs");
}
