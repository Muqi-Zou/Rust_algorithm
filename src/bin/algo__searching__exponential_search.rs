#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::searching::exponential_search as source;

fn main() {
    println!("searching/exponential_search.rs");
}
