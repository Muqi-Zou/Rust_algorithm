#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::backtracking::graph_coloring as source;

fn main() {
    println!("backtracking/graph_coloring.rs");
}
