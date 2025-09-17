#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::ciphers::sha3 as source;

fn main() {
    println!("ciphers/sha3.rs");
}
