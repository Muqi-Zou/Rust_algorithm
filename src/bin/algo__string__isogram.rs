#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::isogram as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("isogram_simple", source::tests::isogram_simple as fn()),
        ("isogram_case_insensitive", source::tests::isogram_case_insensitive as fn()),
        ("isogram_with_spaces", source::tests::isogram_with_spaces as fn()),
        ("isogram_mixed", source::tests::isogram_mixed as fn()),
        ("isogram_long", source::tests::isogram_long as fn()),
        ("isogram_german_city", source::tests::isogram_german_city as fn()),
        ("perfect_pangram", source::tests::perfect_pangram as fn()),
        ("isogram_sentences", source::tests::isogram_sentences as fn()),
        ("isogram_french", source::tests::isogram_french as fn()),
        ("isogram_portuguese", source::tests::isogram_portuguese as fn()),
        ("isogram_spanis", source::tests::isogram_spanis as fn()),
        ("invalid_isogram_with_repeated_char", source::tests::invalid_isogram_with_repeated_char as fn()),
        ("invalid_isogram_with_numbers", source::tests::invalid_isogram_with_numbers as fn()),
        ("invalid_isogram_with_special_char", source::tests::invalid_isogram_with_special_char as fn()),
        ("invalid_isogram_with_comma", source::tests::invalid_isogram_with_comma as fn()),
        ("invalid_isogram_with_spaces", source::tests::invalid_isogram_with_spaces as fn()),
        ("invalid_isogram_with_repeated_phrase", source::tests::invalid_isogram_with_repeated_phrase as fn()),
        ("isogram_empty_string", source::tests::isogram_empty_string as fn()),
        ("isogram_single_character", source::tests::isogram_single_character as fn()),
        ("invalid_isogram_multiple_same_characters", source::tests::invalid_isogram_multiple_same_characters as fn()),
        ("invalid_isogram_with_symbols", source::tests::invalid_isogram_with_symbols as fn()),
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
