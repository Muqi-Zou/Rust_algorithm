#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::geometry::closest_points as source;

fn main() {
    println!("geometry/closest_points.rs");
}
