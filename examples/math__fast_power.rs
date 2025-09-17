#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::all)]

#[path = "src/lib.rs"]
mod source;

pub use source::*;
use source::math::fast_power as target_module;

fn main() {}
