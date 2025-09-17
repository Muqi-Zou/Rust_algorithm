#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::backtracking::knight_tour as source;

fn main() {
    println!("backtracking/knight_tour.rs");
}
