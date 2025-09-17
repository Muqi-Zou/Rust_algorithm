#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::number_theory::kth_factor as source;

fn main() {
    println!("number_theory/kth_factor.rs");
}
