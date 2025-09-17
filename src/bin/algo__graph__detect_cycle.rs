#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::detect_cycle as source;

fn main() {
    println!("graph/detect_cycle.rs");
}
