#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::all)]

#[path = "src/lib.rs"]
mod source;

pub use source::*;
use source::backtracking::all_combination_of_size_k as target_module;

fn main() {}
