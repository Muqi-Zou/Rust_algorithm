#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::sorting::binary_insertion_sort as source;

fn main() {
    println!("sorting/binary_insertion_sort.rs");
}
