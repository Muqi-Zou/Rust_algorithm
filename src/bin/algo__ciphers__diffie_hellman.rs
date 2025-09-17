#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::ciphers::diffie_hellman as source;

fn main() {
    println!("ciphers/diffie_hellman.rs");
}
