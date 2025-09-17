#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::geometry::graham_scan as source;

fn main() {
    println!("geometry/graham_scan.rs");
}
