#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::heavy_light_decomposition as source;

fn main() {
    println!("graph/heavy_light_decomposition.rs");
}
