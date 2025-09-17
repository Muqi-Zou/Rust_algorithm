#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::sylvester_sequence as source;

fn main() {
    println!("math/sylvester_sequence.rs");
}
