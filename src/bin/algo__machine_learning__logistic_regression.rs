#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::machine_learning::logistic_regression as source;

fn main() {
    println!("machine_learning/logistic_regression.rs");
}
