#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::exponential_linear_unit as source;

fn main() {
    println!("math/exponential_linear_unit.rs");
}
