#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::ciphers::base64 as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("pregenerated_random_bytes_encode", source::tests::pregenerated_random_bytes_encode as fn()),
        ("pregenerated_random_bytes_decode", source::tests::pregenerated_random_bytes_decode as fn()),
        ("encode_decode", source::tests::encode_decode as fn()),
        ("decode_encode", source::tests::decode_encode as fn()),
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
