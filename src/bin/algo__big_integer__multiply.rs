#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::big_integer::multiply as source;

fn main() {
    println!("big_integer/multiply.rs");
}
