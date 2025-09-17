#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::sorting::quick_sort_3_ways as source;

fn main() {
    println!("sorting/quick_sort_3_ways.rs");
}
