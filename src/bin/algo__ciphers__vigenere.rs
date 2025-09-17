#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::ciphers::vigenere as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("empty", source::tests::empty as fn()),
        ("vigenere_base", source::tests::vigenere_base as fn()),
        ("vigenere_with_spaces", source::tests::vigenere_with_spaces as fn()),
        ("vigenere_unicode_and_numbers", source::tests::vigenere_unicode_and_numbers as fn()),
        ("vigenere_unicode_key", source::tests::vigenere_unicode_key as fn()),
        ("vigenere_empty_key", source::tests::vigenere_empty_key as fn()),
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
