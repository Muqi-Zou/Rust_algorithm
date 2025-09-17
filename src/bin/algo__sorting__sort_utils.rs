#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[path = "../sorting/sort_utils.rs"]
mod source;

fn main() {
    println!("sorting/sort_utils.rs");
}
