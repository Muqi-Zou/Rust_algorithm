#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::optimal_bst as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_case_1", source::tests::test_case_1 as fn()),
        ("test_case_2", source::tests::test_case_2 as fn()),
        ("test_case_3", source::tests::test_case_3 as fn()),
        ("test_case_4", source::tests::test_case_4 as fn()),
        ("test_case_5", source::tests::test_case_5 as fn()),
        ("test_case_single", source::tests::test_case_single as fn()),
        ("test_case_empty", source::tests::test_case_empty as fn()),
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
