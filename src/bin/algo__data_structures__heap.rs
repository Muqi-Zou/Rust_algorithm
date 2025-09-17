#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::data_structures::heap as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_empty_heap", source::tests::test_empty_heap as fn()),
        ("test_min_heap", source::tests::test_min_heap as fn()),
        ("test_max_heap", source::tests::test_max_heap as fn()),
        ("test_iter_heap", source::tests::test_iter_heap as fn()),
        ("test_from_vec_min", source::tests::test_from_vec_min as fn()),
        ("test_from_vec_max", source::tests::test_from_vec_max as fn()),
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
