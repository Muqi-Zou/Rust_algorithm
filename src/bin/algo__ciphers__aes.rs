#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::ciphers::aes as source;

fn main() {
    println!("ciphers/aes.rs");
}
