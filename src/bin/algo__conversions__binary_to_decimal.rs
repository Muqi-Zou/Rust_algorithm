#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::conversions::binary_to_decimal as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("basic_binary_to_decimal", source::tests::basic_binary_to_decimal as fn()),
        ("big_binary_to_decimal", source::tests::big_binary_to_decimal as fn()),
        ("very_big_binary_to_decimal", source::tests::very_big_binary_to_decimal as fn()),
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
