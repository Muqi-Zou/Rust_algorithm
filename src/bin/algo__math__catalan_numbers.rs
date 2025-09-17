#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::catalan_numbers as source;

fn main() {
    println!("math/catalan_numbers.rs");
}
