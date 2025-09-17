#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::dinic_maxflow as source;

fn main() {
    println!("graph/dinic_maxflow.rs");
}
