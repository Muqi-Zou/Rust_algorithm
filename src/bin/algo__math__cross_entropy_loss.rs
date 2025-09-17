#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::cross_entropy_loss as source;

fn main() {
    println!("math/cross_entropy_loss.rs");
}
