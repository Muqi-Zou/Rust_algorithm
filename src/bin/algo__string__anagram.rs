#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::anagram as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("empty_strings", source::tests::empty_strings as fn()),
        ("empty_and_non_empty", source::tests::empty_and_non_empty as fn()),
        ("single_char_same", source::tests::single_char_same as fn()),
        ("single_char_diff", source::tests::single_char_diff as fn()),
        ("valid_anagram_lowercase", source::tests::valid_anagram_lowercase as fn()),
        ("valid_anagram_with_spaces", source::tests::valid_anagram_with_spaces as fn()),
        ("valid_anagram_mixed_cases", source::tests::valid_anagram_mixed_cases as fn()),
        ("valid_anagram_with_spaces_and_mixed_cases", source::tests::valid_anagram_with_spaces_and_mixed_cases as fn()),
        ("new_york_times", source::tests::new_york_times as fn()),
        ("church_of_scientology", source::tests::church_of_scientology as fn()),
        ("mcdonalds_restaurants", source::tests::mcdonalds_restaurants as fn()),
        ("coronavirus", source::tests::coronavirus as fn()),
        ("synonym_evil", source::tests::synonym_evil as fn()),
        ("synonym_gentleman", source::tests::synonym_gentleman as fn()),
        ("antigram", source::tests::antigram as fn()),
        ("sentences", source::tests::sentences as fn()),
        ("part_of_speech_adj_to_verb", source::tests::part_of_speech_adj_to_verb as fn()),
        ("anagrammatized", source::tests::anagrammatized as fn()),
        ("non_anagram", source::tests::non_anagram as fn()),
        ("invalid_anagram_with_special_char", source::tests::invalid_anagram_with_special_char as fn()),
        ("invalid_anagram_with_numeric_chars", source::tests::invalid_anagram_with_numeric_chars as fn()),
        ("invalid_anagram_with_symbols", source::tests::invalid_anagram_with_symbols as fn()),
        ("non_anagram_length_mismatch", source::tests::non_anagram_length_mismatch as fn()),
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
