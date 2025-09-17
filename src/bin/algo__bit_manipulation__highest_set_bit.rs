#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::bit_manipulation::highest_set_bit as source;

fn main() {
    println!("bit_manipulation/highest_set_bit.rs");
}
