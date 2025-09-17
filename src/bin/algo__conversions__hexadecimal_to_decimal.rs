#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::conversions::hexadecimal_to_decimal as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_hexadecimal_to_decimal_empty", source::tests::test_hexadecimal_to_decimal_empty as fn()),
        ("test_hexadecimal_to_decimal_invalid", source::tests::test_hexadecimal_to_decimal_invalid as fn()),
        ("test_hexadecimal_to_decimal_valid1", source::tests::test_hexadecimal_to_decimal_valid1 as fn()),
        ("test_hexadecimal_to_decimal_valid2", source::tests::test_hexadecimal_to_decimal_valid2 as fn()),
        ("test_hexadecimal_to_decimal_valid3", source::tests::test_hexadecimal_to_decimal_valid3 as fn()),
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
