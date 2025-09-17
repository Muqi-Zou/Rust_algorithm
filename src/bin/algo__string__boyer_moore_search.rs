#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::string::boyer_moore_search as source;

fn main() {
    println!("string/boyer_moore_search.rs");
}
