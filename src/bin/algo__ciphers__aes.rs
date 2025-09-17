#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::ciphers::aes as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_aes_128", source::tests::test_aes_128 as fn()),
        ("test_aes_192", source::tests::test_aes_192 as fn()),
        ("test_aes_256", source::tests::test_aes_256 as fn()),
        ("test_str", source::tests::test_str as fn()),
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
