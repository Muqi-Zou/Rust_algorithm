#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::greatest_common_divisor as source;

fn main() {
    println!("math/greatest_common_divisor.rs");
}
