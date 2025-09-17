#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::string::shortest_palindrome as source;

fn main() {
    println!("string/shortest_palindrome.rs");
}
