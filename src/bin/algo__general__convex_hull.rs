#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::general::convex_hull as source;

fn main() {
    println!("general/convex_hull.rs");
}
