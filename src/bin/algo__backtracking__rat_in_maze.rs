#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::backtracking::rat_in_maze as source;

fn main() {
    println!("backtracking/rat_in_maze.rs");
}
