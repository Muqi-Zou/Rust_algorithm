#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::sorting::tim_sort as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("min_run_length_returns_correct_value", source::tests::min_run_length_returns_correct_value as fn()),
        ("left_and_right_subarrays_into_array", source::tests::left_and_right_subarrays_into_array as fn()),
        ("with_empty_left_subarray", source::tests::with_empty_left_subarray as fn()),
        ("with_empty_right_subarray", source::tests::with_empty_right_subarray as fn()),
        ("with_empty_left_and_right_subarrays", source::tests::with_empty_left_and_right_subarrays as fn()),
        ("sorts_basic_array_correctly", source::tests::sorts_basic_array_correctly as fn()),
        ("sorts_long_array_correctly", source::tests::sorts_long_array_correctly as fn()),
        ("handles_empty_array", source::tests::handles_empty_array as fn()),
        ("handles_single_element_array", source::tests::handles_single_element_array as fn()),
        ("handles_pre_sorted_array", source::tests::handles_pre_sorted_array as fn()),
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
