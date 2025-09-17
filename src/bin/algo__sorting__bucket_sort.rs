#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::sorting::bucket_sort as source;

fn main() {
    println!("sorting/bucket_sort.rs");
}
