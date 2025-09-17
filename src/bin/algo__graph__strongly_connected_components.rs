#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::strongly_connected_components as source;

fn main() {
    println!("graph/strongly_connected_components.rs");
}
