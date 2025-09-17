#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::string::knuth_morris_pratt as source;

fn main() {
    println!("string/knuth_morris_pratt.rs");
}
