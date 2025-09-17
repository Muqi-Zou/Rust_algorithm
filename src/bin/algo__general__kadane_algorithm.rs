#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::general::kadane_algorithm as source;

fn main() {
    println!("general/kadane_algorithm.rs");
}
