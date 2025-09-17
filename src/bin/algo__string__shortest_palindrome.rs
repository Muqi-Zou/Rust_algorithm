#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::shortest_palindrome as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("empty", source::tests::empty as fn()),
        ("extend_left_1", source::tests::extend_left_1 as fn()),
        ("extend_left_2", source::tests::extend_left_2 as fn()),
        ("unicode_1", source::tests::unicode_1 as fn()),
        ("unicode_2", source::tests::unicode_2 as fn()),
        ("single_char", source::tests::single_char as fn()),
        ("already_palindrome", source::tests::already_palindrome as fn()),
        ("extend_left_3", source::tests::extend_left_3 as fn()),
        ("extend_left_4", source::tests::extend_left_4 as fn()),
        ("long_string", source::tests::long_string as fn()),
        ("repetitive", source::tests::repetitive as fn()),
        ("complex", source::tests::complex as fn()),
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
