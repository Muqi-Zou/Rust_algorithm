#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::ciphers::sha256 as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_constants", source::tests::test_constants as fn()),
        ("empty", source::tests::empty as fn()),
        ("ascii", source::tests::ascii as fn()),
        ("ascii_avalanche", source::tests::ascii_avalanche as fn()),
        ("long_ascii", source::tests::long_ascii as fn()),
        ("short_ascii", source::tests::short_ascii as fn()),
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
