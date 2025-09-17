#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::all)]

#[path = "src/lib.rs"]
mod source;

pub use source::*;
use source::searching::ternary_search_min_max as target_module;

fn main() {}
