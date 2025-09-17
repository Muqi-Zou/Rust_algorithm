#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::bit_manipulation::n_bits_gray_code as source;

fn main() {
    println!("bit_manipulation/n_bits_gray_code.rs");
}
