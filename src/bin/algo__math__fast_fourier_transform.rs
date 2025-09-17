#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::fast_fourier_transform as source;

fn main() {
    println!("math/fast_fourier_transform.rs");
}
