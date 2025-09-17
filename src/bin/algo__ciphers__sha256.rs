#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::ciphers::sha256 as source;

fn main() {
    println!("ciphers/sha256.rs");
}
