#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::string::suffix_array as source;

fn main() {
    println!("string/suffix_array.rs");
}
