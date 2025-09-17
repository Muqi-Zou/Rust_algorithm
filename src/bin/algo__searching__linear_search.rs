#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::searching::linear_search as source;

fn main() {
    println!("searching/linear_search.rs");
}
