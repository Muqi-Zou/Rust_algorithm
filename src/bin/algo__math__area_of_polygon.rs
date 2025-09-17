#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::area_of_polygon as source;

fn main() {
    println!("math/area_of_polygon.rs");
}
