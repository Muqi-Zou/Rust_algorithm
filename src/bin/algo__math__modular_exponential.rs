#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::modular_exponential as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_modular_exponential_positive", source::tests::test_modular_exponential_positive as fn()),
        ("test_modular_inverse", source::tests::test_modular_inverse as fn()),
        ("test_modular_exponential_negative", source::tests::test_modular_exponential_negative as fn()),
        ("test_modular_exponential_edge_cases", source::tests::test_modular_exponential_edge_cases as fn()),
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
