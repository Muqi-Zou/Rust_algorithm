#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::lcm_of_n_numbers as source;

fn main() {
    println!("math/lcm_of_n_numbers.rs");
}
