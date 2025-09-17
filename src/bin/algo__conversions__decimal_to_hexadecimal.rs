#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::conversions::decimal_to_hexadecimal as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_zero", source::tests::test_zero as fn()),
        ("test_single_digit_decimal", source::tests::test_single_digit_decimal as fn()),
        ("test_single_digit_hexadecimal", source::tests::test_single_digit_hexadecimal as fn()),
        ("test_multiple_digit_hexadecimal", source::tests::test_multiple_digit_hexadecimal as fn()),
        ("test_big", source::tests::test_big as fn()),
        ("test_random", source::tests::test_random as fn()),
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
