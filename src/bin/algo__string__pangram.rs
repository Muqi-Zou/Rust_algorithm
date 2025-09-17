#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::pangram as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_not_pangram_simple", source::tests::test_not_pangram_simple as fn()),
        ("test_not_pangram_day", source::tests::test_not_pangram_day as fn()),
        ("test_not_pangram_almost", source::tests::test_not_pangram_almost as fn()),
        ("test_pangram_standard", source::tests::test_pangram_standard as fn()),
        ("test_pangram_boxer", source::tests::test_pangram_boxer as fn()),
        ("test_pangram_discotheques", source::tests::test_pangram_discotheques as fn()),
        ("test_pangram_zebras", source::tests::test_pangram_zebras as fn()),
        ("test_perfect_pangram_jock", source::tests::test_perfect_pangram_jock as fn()),
        ("test_empty_string", source::tests::test_empty_string as fn()),
        ("test_repeated_letter", source::tests::test_repeated_letter as fn()),
        ("test_non_alphabetic", source::tests::test_non_alphabetic as fn()),
        ("test_mixed_case_pangram", source::tests::test_mixed_case_pangram as fn()),
        ("test_perfect_pangram_with_symbols", source::tests::test_perfect_pangram_with_symbols as fn()),
        ("test_long_non_pangram", source::tests::test_long_non_pangram as fn()),
        ("test_near_pangram_missing_one_letter", source::tests::test_near_pangram_missing_one_letter as fn()),
        ("test_near_pangram_missing_two_letters", source::tests::test_near_pangram_missing_two_letters as fn()),
        ("test_near_pangram_with_special_characters", source::tests::test_near_pangram_with_special_characters as fn()),
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
