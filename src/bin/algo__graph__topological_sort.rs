#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::topological_sort as source;

fn main() {
    println!("graph/topological_sort.rs");
}
