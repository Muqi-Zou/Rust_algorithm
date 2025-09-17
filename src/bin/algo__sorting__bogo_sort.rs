#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::sorting::bogo_sort as source;

fn main() {
    println!("sorting/bogo_sort.rs");
}
