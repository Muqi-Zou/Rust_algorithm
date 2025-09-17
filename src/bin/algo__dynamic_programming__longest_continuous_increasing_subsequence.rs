#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::longest_continuous_increasing_subsequence as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("empty_array", source::tests::empty_array as fn()),
        ("single_element", source::tests::single_element as fn()),
        ("all_increasing", source::tests::all_increasing as fn()),
        ("all_decreasing", source::tests::all_decreasing as fn()),
        ("with_equal_elements", source::tests::with_equal_elements as fn()),
        ("increasing_with_plateau", source::tests::increasing_with_plateau as fn()),
        ("mixed_elements", source::tests::mixed_elements as fn()),
        ("alternating_increase_decrease", source::tests::alternating_increase_decrease as fn()),
        ("zigzag", source::tests::zigzag as fn()),
        ("single_negative_element", source::tests::single_negative_element as fn()),
        ("negative_and_positive_mixed", source::tests::negative_and_positive_mixed as fn()),
        ("increasing_then_decreasing", source::tests::increasing_then_decreasing as fn()),
        ("single_increasing_subsequence_later", source::tests::single_increasing_subsequence_later as fn()),
        ("longer_subsequence_at_start", source::tests::longer_subsequence_at_start as fn()),
        ("longer_subsequence_at_end", source::tests::longer_subsequence_at_end as fn()),
        ("longest_subsequence_at_start", source::tests::longest_subsequence_at_start as fn()),
        ("longest_subsequence_at_end", source::tests::longest_subsequence_at_end as fn()),
        ("repeated_elements", source::tests::repeated_elements as fn()),
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
