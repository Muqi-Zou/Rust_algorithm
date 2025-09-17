#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::conversions::hexadecimal_to_binary as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_empty_string", source::tests::test_empty_string as fn()),
        ("test_hexadecimal", source::tests::test_hexadecimal as fn()),
        ("test_hexadecimal2", source::tests::test_hexadecimal2 as fn()),
        ("test_invalid_hexadecimal", source::tests::test_invalid_hexadecimal as fn()),
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
