#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::navigation::haversine as source;

fn main() {
    println!("navigation/haversine.rs");
}
