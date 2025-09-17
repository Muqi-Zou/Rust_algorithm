#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::palindrome as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("odd_palindrome", source::tests::odd_palindrome as fn()),
        ("even_palindrome", source::tests::even_palindrome as fn()),
        ("single_character_palindrome", source::tests::single_character_palindrome as fn()),
        ("single_word_palindrome", source::tests::single_word_palindrome as fn()),
        ("case_insensitive_palindrome", source::tests::case_insensitive_palindrome as fn()),
        ("mixed_case_and_punctuation_palindrome", source::tests::mixed_case_and_punctuation_palindrome as fn()),
        ("mixed_case_and_space_palindrome", source::tests::mixed_case_and_space_palindrome as fn()),
        ("empty_string", source::tests::empty_string as fn()),
        ("pompeii_palindrome", source::tests::pompeii_palindrome as fn()),
        ("napoleon_palindrome", source::tests::napoleon_palindrome as fn()),
        ("john_taylor_palindrome", source::tests::john_taylor_palindrome as fn()),
        ("well_know_english_palindrome", source::tests::well_know_english_palindrome as fn()),
        ("palindromic_phrase", source::tests::palindromic_phrase as fn()),
        ("names_palindrome", source::tests::names_palindrome as fn()),
        ("prime_minister_of_cambodia", source::tests::prime_minister_of_cambodia as fn()),
        ("japanese_novelist_and_manga_writer", source::tests::japanese_novelist_and_manga_writer as fn()),
        ("actor", source::tests::actor as fn()),
        ("rock_vocalist", source::tests::rock_vocalist as fn()),
        ("pokemon_species", source::tests::pokemon_species as fn()),
        ("lychrel_num_56", source::tests::lychrel_num_56 as fn()),
        ("universal_palindrome_date", source::tests::universal_palindrome_date as fn()),
        ("french_palindrome", source::tests::french_palindrome as fn()),
        ("finnish_palindrome", source::tests::finnish_palindrome as fn()),
        ("non_palindrome_simple", source::tests::non_palindrome_simple as fn()),
        ("non_palindrome_with_punctuation", source::tests::non_palindrome_with_punctuation as fn()),
        ("non_palindrome_mixed_case", source::tests::non_palindrome_mixed_case as fn()),
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
