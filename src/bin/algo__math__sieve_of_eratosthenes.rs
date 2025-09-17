#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::math::sieve_of_eratosthenes as source;

fn main() {
    println!("math/sieve_of_eratosthenes.rs");
}
