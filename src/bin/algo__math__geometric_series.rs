#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::geometric_series as source;

fn main() {
    println!("math/geometric_series.rs");
}
