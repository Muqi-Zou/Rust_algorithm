#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::graph_enumeration as source;

fn main() {
    println!("graph/graph_enumeration.rs");
}
