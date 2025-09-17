#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::string::suffix_array_manber_myers as source;

fn main() {
    println!("string/suffix_array_manber_myers.rs");
}
