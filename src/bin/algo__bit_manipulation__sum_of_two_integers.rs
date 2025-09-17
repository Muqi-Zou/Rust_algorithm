#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::bit_manipulation::sum_of_two_integers as source;

fn main() {
    println!("bit_manipulation/sum_of_two_integers.rs");
}
