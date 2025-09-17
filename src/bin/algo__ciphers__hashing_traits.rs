#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::ciphers::hashing_traits as source;

fn main() {
    println!("ciphers/hashing_traits.rs");
}
