#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::searching::quick_select as source;

fn main() {
    println!("searching/quick_select.rs");
}
