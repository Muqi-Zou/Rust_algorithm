#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::simpsons_integration as source;

fn main() {
    println!("math/simpsons_integration.rs");
}
