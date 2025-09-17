#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::tarjans_ssc as source;

fn main() {
    println!("graph/tarjans_ssc.rs");
}
