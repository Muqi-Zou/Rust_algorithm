#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::all)]

#[path = "src/lib.rs"]
mod source;

pub use source::*;
use source::big_integer::poly1305 as target_module;

fn main() {}
