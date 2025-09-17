#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::graph::lee_breadth_first_search as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_lee_exists", source::tests::test_lee_exists as fn()),
        ("test_lee_does_not_exist", source::tests::test_lee_does_not_exist as fn()),
        ("test_source_equals_destination", source::tests::test_source_equals_destination as fn()),
        ("test_lee_exists_2", source::tests::test_lee_exists_2 as fn()),
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
