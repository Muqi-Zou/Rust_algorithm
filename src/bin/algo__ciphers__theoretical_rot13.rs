#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::ciphers::theoretical_rot13 as source;

fn main() {
    println!("ciphers/theoretical_rot13.rs");
}
