#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::ciphers::theoretical_rot13 as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_single_letter", source::tests::test_single_letter as fn()),
        ("test_bunch_of_letters", source::tests::test_bunch_of_letters as fn()),
        ("test_non_ascii", source::tests::test_non_ascii as fn()),
        ("test_twice", source::tests::test_twice as fn()),
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
