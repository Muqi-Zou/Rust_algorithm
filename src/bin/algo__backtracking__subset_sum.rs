#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::backtracking::subset_sum as source;

fn main() {
    println!("backtracking/subset_sum.rs");
}
