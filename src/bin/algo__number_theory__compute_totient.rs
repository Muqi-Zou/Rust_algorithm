#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::number_theory::compute_totient as source;

fn main() {
    println!("number_theory/compute_totient.rs");
}
