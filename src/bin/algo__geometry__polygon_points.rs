#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::geometry::polygon_points as source;

fn main() {
    println!("geometry/polygon_points.rs");
}
