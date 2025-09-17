#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::searching::ternary_search_min_max_recursive as source;

fn main() {
    println!("searching/ternary_search_min_max_recursive.rs");
}
