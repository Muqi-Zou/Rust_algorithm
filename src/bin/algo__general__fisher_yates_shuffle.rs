#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::general::fisher_yates_shuffle as source;

fn main() {
    println!("general/fisher_yates_shuffle.rs");
}
