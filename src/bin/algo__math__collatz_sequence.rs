#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::collatz_sequence as source;

fn main() {
    println!("math/collatz_sequence.rs");
}
