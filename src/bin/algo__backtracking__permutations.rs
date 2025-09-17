#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::backtracking::permutations as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_permute_basic", source::tests::test_permute_basic as fn()),
        ("test_permute_empty", source::tests::test_permute_empty as fn()),
        ("test_permute_single", source::tests::test_permute_single as fn()),
        ("test_permute_duplicates", source::tests::test_permute_duplicates as fn()),
        ("test_permute_all_duplicates", source::tests::test_permute_all_duplicates as fn()),
        ("test_permute_negative", source::tests::test_permute_negative as fn()),
        ("test_permute_mixed", source::tests::test_permute_mixed as fn()),
        ("test_permute_larger", source::tests::test_permute_larger as fn()),
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
