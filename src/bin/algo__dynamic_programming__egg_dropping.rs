#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::dynamic_programming::egg_dropping as source;

fn main() {
    println!("dynamic_programming/egg_dropping.rs");
}
