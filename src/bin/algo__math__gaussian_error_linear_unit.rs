#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::gaussian_error_linear_unit as source;

fn main() {
    println!("math/gaussian_error_linear_unit.rs");
}
