#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::backtracking::all_combination_of_size_k as source;

fn main() {
    println!("backtracking/all_combination_of_size_k.rs");
}
