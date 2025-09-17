#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::general::kadane_algorithm as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_kadanes_algorithm_positive", source::tests::test_kadanes_algorithm_positive as fn()),
        ("test_kadanes_algorithm_negative", source::tests::test_kadanes_algorithm_negative as fn()),
        ("test_kadanes_algorithm_mixed", source::tests::test_kadanes_algorithm_mixed as fn()),
        ("test_kadanes_algorithm_empty", source::tests::test_kadanes_algorithm_empty as fn()),
        ("test_kadanes_algorithm_single_positive", source::tests::test_kadanes_algorithm_single_positive as fn()),
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
