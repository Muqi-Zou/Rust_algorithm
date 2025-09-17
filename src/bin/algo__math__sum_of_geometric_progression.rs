#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::sum_of_geometric_progression as source;

fn main() {
    println!("math/sum_of_geometric_progression.rs");
}
