#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::quadratic_residue as source;

fn main() {
    println!("math/quadratic_residue.rs");
}
