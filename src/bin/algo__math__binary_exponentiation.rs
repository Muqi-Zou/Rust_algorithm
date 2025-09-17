#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::binary_exponentiation as source;

fn main() {
    println!("math/binary_exponentiation.rs");
}
