#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::all)]

#[path = "src/lib.rs"]
mod source;

pub use source::*;
use source::general::permutations::steinhaus_johnson_trotter as target_module;

fn main() {}
