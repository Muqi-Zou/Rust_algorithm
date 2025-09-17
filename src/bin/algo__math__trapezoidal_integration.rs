#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::trapezoidal_integration as source;

fn main() {
    println!("math/trapezoidal_integration.rs");
}
