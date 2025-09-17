#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::maximum_subarray as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_all_non_negative", source::tests::test_all_non_negative as fn()),
        ("test_all_negative", source::tests::test_all_negative as fn()),
        ("test_mixed_negative_and_positive", source::tests::test_mixed_negative_and_positive as fn()),
        ("test_single_element_positive", source::tests::test_single_element_positive as fn()),
        ("test_single_element_negative", source::tests::test_single_element_negative as fn()),
        ("test_mixed_elements", source::tests::test_mixed_elements as fn()),
        ("test_empty_array", source::tests::test_empty_array as fn()),
        ("test_all_zeroes", source::tests::test_all_zeroes as fn()),
        ("test_single_zero", source::tests::test_single_zero as fn()),
        ("test_alternating_signs", source::tests::test_alternating_signs as fn()),
        ("test_all_negatives_with_one_positive", source::tests::test_all_negatives_with_one_positive as fn()),
        ("test_all_positives_with_one_negative", source::tests::test_all_positives_with_one_negative as fn()),
        ("test_all_positives", source::tests::test_all_positives as fn()),
        ("test_large_values", source::tests::test_large_values as fn()),
        ("test_large_array", source::tests::test_large_array as fn()),
        ("test_large_negative_array", source::tests::test_large_negative_array as fn()),
        ("test_single_large_positive", source::tests::test_single_large_positive as fn()),
        ("test_single_large_negative", source::tests::test_single_large_negative as fn()),
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
