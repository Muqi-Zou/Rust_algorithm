#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::data_structures::hash_table as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_insert_and_search", source::tests::test_insert_and_search as fn()),
        ("test_resize", source::tests::test_resize as fn()),
        ("test_search_nonexistent", source::tests::test_search_nonexistent as fn()),
        ("test_multiple_inserts_and_searches", source::tests::test_multiple_inserts_and_searches as fn()),
        ("test_not_overwrite_existing_key", source::tests::test_not_overwrite_existing_key as fn()),
        ("test_empty_search", source::tests::test_empty_search as fn()),
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
