#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::dynamic_programming::maximum_subarray as source;

fn main() {
    println!("dynamic_programming/maximum_subarray.rs");
}
