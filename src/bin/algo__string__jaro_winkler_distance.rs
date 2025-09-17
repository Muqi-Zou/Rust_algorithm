#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::string::jaro_winkler_distance as source;

fn main() {
    println!("string/jaro_winkler_distance.rs");
}
