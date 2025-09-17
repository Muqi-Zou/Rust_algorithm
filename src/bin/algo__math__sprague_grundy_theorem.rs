#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::sprague_grundy_theorem as source;

fn main() {
    println!("math/sprague_grundy_theorem.rs");
}
