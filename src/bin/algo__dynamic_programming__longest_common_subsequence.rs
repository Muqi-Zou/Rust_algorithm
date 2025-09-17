#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::longest_common_subsequence as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("empty_case", source::tests::empty_case as fn()),
        ("one_empty", source::tests::one_empty as fn()),
        ("identical_strings", source::tests::identical_strings as fn()),
        ("completely_different", source::tests::completely_different as fn()),
        ("single_character", source::tests::single_character as fn()),
        ("different_length", source::tests::different_length as fn()),
        ("special_characters", source::tests::special_characters as fn()),
        ("long_strings", source::tests::long_strings as fn()),
        ("unicode_characters", source::tests::unicode_characters as fn()),
        ("spaces_and_punctuation_0", source::tests::spaces_and_punctuation_0 as fn()),
        ("spaces_and_punctuation_1", source::tests::spaces_and_punctuation_1 as fn()),
        ("random_case_1", source::tests::random_case_1 as fn()),
        ("random_case_2", source::tests::random_case_2 as fn()),
        ("random_case_3", source::tests::random_case_3 as fn()),
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
