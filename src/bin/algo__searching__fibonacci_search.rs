#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::searching::fibonacci_search as source;

fn main() {
    println!("searching/fibonacci_search.rs");
}
