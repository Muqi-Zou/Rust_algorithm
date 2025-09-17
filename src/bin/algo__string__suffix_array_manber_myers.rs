#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::suffix_array_manber_myers as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_suffix_array", source::tests::test_suffix_array as fn()),
        ("test_empty_string", source::tests::test_empty_string as fn()),
        ("test_single_character", source::tests::test_single_character as fn()),
        ("test_repeating_characters", source::tests::test_repeating_characters as fn()),
        ("test_long_string", source::tests::test_long_string as fn()),
        ("test_mix_of_characters", source::tests::test_mix_of_characters as fn()),
        ("test_whitespace_characters", source::tests::test_whitespace_characters as fn()),
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
