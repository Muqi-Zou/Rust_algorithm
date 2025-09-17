#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::depth_first_search_tic_tac_toe as source;

fn main() {
    println!("graph/depth_first_search_tic_tac_toe.rs");
}
