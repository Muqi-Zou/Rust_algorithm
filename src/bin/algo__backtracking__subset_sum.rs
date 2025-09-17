#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::backtracking::subset_sum as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_small_set_with_sum", source::tests::test_small_set_with_sum as fn()),
        ("test_small_set_without_sum", source::tests::test_small_set_without_sum as fn()),
        ("test_consecutive_set_with_sum", source::tests::test_consecutive_set_with_sum as fn()),
        ("test_consecutive_set_without_sum", source::tests::test_consecutive_set_without_sum as fn()),
        ("test_large_set_with_sum", source::tests::test_large_set_with_sum as fn()),
        ("test_empty_set", source::tests::test_empty_set as fn()),
        ("test_empty_set_with_nonzero_sum", source::tests::test_empty_set_with_nonzero_sum as fn()),
        ("test_single_element_equal_to_sum", source::tests::test_single_element_equal_to_sum as fn()),
        ("test_single_element_not_equal_to_sum", source::tests::test_single_element_not_equal_to_sum as fn()),
        ("test_negative_set_with_sum", source::tests::test_negative_set_with_sum as fn()),
        ("test_negative_sum", source::tests::test_negative_sum as fn()),
        ("test_negative_sum_with_negatives", source::tests::test_negative_sum_with_negatives as fn()),
        ("test_negative_sum_with_negatives_no_solution", source::tests::test_negative_sum_with_negatives_no_solution as fn()),
        ("test_even_inputs_odd_target", source::tests::test_even_inputs_odd_target as fn()),
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
