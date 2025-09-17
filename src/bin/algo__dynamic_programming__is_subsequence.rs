#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::is_subsequence as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_empty_subsequence", source::tests::test_empty_subsequence as fn()),
        ("test_empty_strings", source::tests::test_empty_strings as fn()),
        ("test_non_empty_sub_empty_main", source::tests::test_non_empty_sub_empty_main as fn()),
        ("test_subsequence_found", source::tests::test_subsequence_found as fn()),
        ("test_subsequence_not_found", source::tests::test_subsequence_not_found as fn()),
        ("test_longer_sub", source::tests::test_longer_sub as fn()),
        ("test_single_character_match", source::tests::test_single_character_match as fn()),
        ("test_single_character_not_match", source::tests::test_single_character_not_match as fn()),
        ("test_subsequence_at_start", source::tests::test_subsequence_at_start as fn()),
        ("test_subsequence_at_end", source::tests::test_subsequence_at_end as fn()),
        ("test_same_characters", source::tests::test_same_characters as fn()),
        ("test_interspersed_subsequence", source::tests::test_interspersed_subsequence as fn()),
        ("test_different_chars_in_subsequence", source::tests::test_different_chars_in_subsequence as fn()),
        ("test_single_character_in_main_not_match", source::tests::test_single_character_in_main_not_match as fn()),
        ("test_single_character_in_main_match", source::tests::test_single_character_in_main_match as fn()),
        ("test_subsequence_with_special_chars", source::tests::test_subsequence_with_special_chars as fn()),
        ("test_case_sensitive", source::tests::test_case_sensitive as fn()),
        ("test_subsequence_with_whitespace", source::tests::test_subsequence_with_whitespace as fn()),
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
