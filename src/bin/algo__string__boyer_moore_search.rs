#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::boyer_moore_search as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_simple_match", source::tests::test_simple_match as fn()),
        ("test_no_match", source::tests::test_no_match as fn()),
        ("test_partial_match", source::tests::test_partial_match as fn()),
        ("test_empty_text", source::tests::test_empty_text as fn()),
        ("test_empty_pattern", source::tests::test_empty_pattern as fn()),
        ("test_both_empty", source::tests::test_both_empty as fn()),
        ("test_pattern_longer_than_text", source::tests::test_pattern_longer_than_text as fn()),
        ("test_single_character_text", source::tests::test_single_character_text as fn()),
        ("test_single_character_pattern", source::tests::test_single_character_pattern as fn()),
        ("test_case_sensitivity", source::tests::test_case_sensitivity as fn()),
        ("test_overlapping_patterns", source::tests::test_overlapping_patterns as fn()),
        ("test_special_characters", source::tests::test_special_characters as fn()),
        ("test_numerical_pattern", source::tests::test_numerical_pattern as fn()),
        ("test_partial_overlap_no_match", source::tests::test_partial_overlap_no_match as fn()),
        ("test_single_occurrence", source::tests::test_single_occurrence as fn()),
        ("test_single_occurrence_with_noise", source::tests::test_single_occurrence_with_noise as fn()),
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
