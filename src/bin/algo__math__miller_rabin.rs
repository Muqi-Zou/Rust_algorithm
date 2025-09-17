#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::miller_rabin as source;

fn main() {
    println!("math/miller_rabin.rs");
}
