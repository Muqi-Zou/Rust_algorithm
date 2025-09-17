#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::longest_common_substring as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_empty_strings", source::tests::test_empty_strings as fn()),
        ("test_one_empty_string", source::tests::test_one_empty_string as fn()),
        ("test_identical_single_char", source::tests::test_identical_single_char as fn()),
        ("test_different_single_char", source::tests::test_different_single_char as fn()),
        ("test_common_substring_at_start", source::tests::test_common_substring_at_start as fn()),
        ("test_common_substring_at_middle", source::tests::test_common_substring_at_middle as fn()),
        ("test_common_substring_at_end", source::tests::test_common_substring_at_end as fn()),
        ("test_no_common_substring", source::tests::test_no_common_substring as fn()),
        ("test_overlapping_substrings", source::tests::test_overlapping_substrings as fn()),
        ("test_special_characters", source::tests::test_special_characters as fn()),
        ("test_case_sensitive", source::tests::test_case_sensitive as fn()),
        ("test_full_string_match", source::tests::test_full_string_match as fn()),
        ("test_substring_with_repeated_chars", source::tests::test_substring_with_repeated_chars as fn()),
        ("test_longer_strings_with_common_substring", source::tests::test_longer_strings_with_common_substring as fn()),
        ("test_no_common_substring_with_special_chars", source::tests::test_no_common_substring_with_special_chars as fn()),
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
