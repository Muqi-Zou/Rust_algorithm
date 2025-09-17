#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::eulerian_path as source;

fn main() {
    println!("graph/eulerian_path.rs");
}
