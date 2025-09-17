#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::general::permutations as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_valid_permutations", source::tests::test_valid_permutations as fn()),
        ("test_invalid_permutation_1", source::tests::test_invalid_permutation_1 as fn()),
        ("test_invalid_permutation_2", source::tests::test_invalid_permutation_2 as fn()),
        ("test_invalid_permutation_3", source::tests::test_invalid_permutation_3 as fn()),
        ("test_invalid_permutation_repeat", source::tests::test_invalid_permutation_repeat as fn()),
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
