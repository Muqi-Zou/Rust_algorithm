#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::compression::move_to_front as source;

fn main() {
    println!("compression/move_to_front.rs");
}
