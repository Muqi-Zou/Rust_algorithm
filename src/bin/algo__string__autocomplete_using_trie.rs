#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::string::autocomplete_using_trie as source;

fn main() {
    println!("string/autocomplete_using_trie.rs");
}
