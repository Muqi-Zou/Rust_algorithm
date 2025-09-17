#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::minimum_spanning_tree as source;

fn main() {
    println!("graph/minimum_spanning_tree.rs");
}
