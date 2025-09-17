#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::run_length_encoding as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("empty_input", source::tests::empty_input as fn()),
        ("repeated_char", source::tests::repeated_char as fn()),
        ("no_repeated", source::tests::no_repeated as fn()),
        ("regular_input", source::tests::regular_input as fn()),
        ("two_blocks_with_same_char", source::tests::two_blocks_with_same_char as fn()),
        ("long_input", source::tests::long_input as fn()),
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
