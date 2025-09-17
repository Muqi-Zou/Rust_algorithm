#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::field as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_field_elements", source::tests::test_field_elements as fn()),
        ("large_prime_field", source::tests::large_prime_field as fn()),
        ("inverse", source::tests::inverse as fn()),
        ("test_mod_inverse", source::tests::test_mod_inverse as fn()),
        ("integer_mul", source::tests::integer_mul as fn()),
        ("from_integer", source::tests::from_integer as fn()),
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
