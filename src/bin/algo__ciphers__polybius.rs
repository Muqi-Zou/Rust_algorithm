#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::ciphers::polybius as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("encode_empty", source::tests::encode_empty as fn()),
        ("encode_valid_string", source::tests::encode_valid_string as fn()),
        ("encode_emoji", source::tests::encode_emoji as fn()),
        ("decode_empty", source::tests::decode_empty as fn()),
        ("decode_valid_string", source::tests::decode_valid_string as fn()),
        ("decode_emoji", source::tests::decode_emoji as fn()),
        ("decode_string_with_whitespace", source::tests::decode_string_with_whitespace as fn()),
        ("decode_unknown_string", source::tests::decode_unknown_string as fn()),
        ("decode_odd_length", source::tests::decode_odd_length as fn()),
        ("encode_and_decode", source::tests::encode_and_decode as fn()),
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
