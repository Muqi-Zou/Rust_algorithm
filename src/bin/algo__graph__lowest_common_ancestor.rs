#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::lowest_common_ancestor as source;

fn main() {
    println!("graph/lowest_common_ancestor.rs");
}
