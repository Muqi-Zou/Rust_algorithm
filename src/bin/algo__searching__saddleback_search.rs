#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::searching::saddleback_search as source;

fn main() {
    println!("searching/saddleback_search.rs");
}
