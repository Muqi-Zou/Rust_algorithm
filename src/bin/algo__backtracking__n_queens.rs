#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::backtracking::n_queens as source;

fn main() {
    println!("backtracking/n_queens.rs");
}
