#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::backtracking::hamiltonian_cycle as source;

fn main() {
    println!("backtracking/hamiltonian_cycle.rs");
}
