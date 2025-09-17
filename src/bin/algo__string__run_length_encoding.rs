#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::string::run_length_encoding as source;

fn main() {
    println!("string/run_length_encoding.rs");
}
