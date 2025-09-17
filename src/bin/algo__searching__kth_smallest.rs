#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::searching::kth_smallest as source;

fn main() {
    println!("searching/kth_smallest.rs");
}
