#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::disjoint_set_union as source;

fn main() {
    println!("graph/disjoint_set_union.rs");
}
