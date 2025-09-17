#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::reverse as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_simple_palindrome", source::tests::test_simple_palindrome as fn()),
        ("test_non_palindrome", source::tests::test_non_palindrome as fn()),
        ("test_sentence_with_spaces", source::tests::test_sentence_with_spaces as fn()),
        ("test_empty_string", source::tests::test_empty_string as fn()),
        ("test_single_character", source::tests::test_single_character as fn()),
        ("test_leading_trailing_spaces", source::tests::test_leading_trailing_spaces as fn()),
        ("test_unicode_characters", source::tests::test_unicode_characters as fn()),
        ("test_mixed_content", source::tests::test_mixed_content as fn()),
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
