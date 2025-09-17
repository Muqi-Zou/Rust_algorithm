#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::greedy::stable_matching as source;

fn main() {
    println!("greedy/stable_matching.rs");
}
