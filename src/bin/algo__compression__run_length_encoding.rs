#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::compression::run_length_encoding as source;

fn main() {
    println!("compression/run_length_encoding.rs");
}
