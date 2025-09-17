#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::data_structures::treap as source;

fn main() {
    println!("data_structures/treap.rs");
}
