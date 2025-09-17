#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::geometry::jarvis_scan as source;

fn main() {
    println!("geometry/jarvis_scan.rs");
}
