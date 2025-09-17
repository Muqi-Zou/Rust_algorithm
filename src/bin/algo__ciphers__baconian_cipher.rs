#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::ciphers::baconian_cipher as source;

fn main() {
    println!("ciphers/baconian_cipher.rs");
}
