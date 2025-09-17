#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::armstrong_number as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("one_digit_armstrong_number", source::tests::one_digit_armstrong_number as fn()),
        ("two_digit_numbers_are_not_armstrong_numbers", source::tests::two_digit_numbers_are_not_armstrong_numbers as fn()),
        ("three_digit_armstrong_number", source::tests::three_digit_armstrong_number as fn()),
        ("three_digit_non_armstrong_number", source::tests::three_digit_non_armstrong_number as fn()),
        ("big_armstrong_number", source::tests::big_armstrong_number as fn()),
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
