#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::searching::moore_voting as source;

fn main() {
    println!("searching/moore_voting.rs");
}
