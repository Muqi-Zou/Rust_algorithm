#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::data_structures::trie as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_insertion_and_retrieval_with_strings", source::tests::test_insertion_and_retrieval_with_strings as fn()),
        ("test_insertion_and_retrieval_with_integers", source::tests::test_insertion_and_retrieval_with_integers as fn()),
        ("test_empty_trie", source::tests::test_empty_trie as fn()),
        ("test_insert_empty_key", source::tests::test_insert_empty_key as fn()),
        ("test_overlapping_keys", source::tests::test_overlapping_keys as fn()),
        ("test_partial_match", source::tests::test_partial_match as fn()),
    ];
    for (name, test) in TESTS {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| test()));
        executed += 1;
        match result {
            Ok(_) => println!("[ok] {}::tests::{}", core::module_path!(), name),
            Err(_) => {
                println!("[failed] {}::tests::{}", core::module_path!(), name);
                failures += 1;
            }
        }
    }
    if executed == 0 {
        println!("No tests discovered.");
    }
    if failures > 0 {
        std::process::exit(1);
    }
}
