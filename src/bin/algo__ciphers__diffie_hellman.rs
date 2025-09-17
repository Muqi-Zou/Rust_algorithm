#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::ciphers::diffie_hellman as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("verify_invalid_pub_key", source::tests::verify_invalid_pub_key as fn()),
        ("verify_valid_pub_key", source::tests::verify_valid_pub_key as fn()),
        ("verify_invalid_pub_key_same_as_prime", source::tests::verify_invalid_pub_key_same_as_prime as fn()),
        ("verify_key_exchange", source::tests::verify_key_exchange as fn()),
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
