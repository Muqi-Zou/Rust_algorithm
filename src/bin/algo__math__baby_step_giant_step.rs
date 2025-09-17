#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::baby_step_giant_step as source;

fn main() {
    println!("math/baby_step_giant_step.rs");
}
