#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::centroid_decomposition as source;

fn main() {
    println!("graph/centroid_decomposition.rs");
}
