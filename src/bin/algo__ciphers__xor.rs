#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::ciphers::xor as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_simple", source::tests::test_simple as fn()),
        ("test_every_alphabet_with_space", source::tests::test_every_alphabet_with_space as fn()),
        ("test_multi_byte", source::tests::test_multi_byte as fn()),
        ("test_zero_byte", source::tests::test_zero_byte as fn()),
        ("test_invalid_byte", source::tests::test_invalid_byte as fn()),
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
