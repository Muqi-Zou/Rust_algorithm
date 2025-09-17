#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::machine_learning::loss_function as source;

fn main() {
    println!("machine_learning/loss_function/mod.rs");
}
