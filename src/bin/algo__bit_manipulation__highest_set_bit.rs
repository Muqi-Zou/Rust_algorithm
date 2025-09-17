#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::bit_manipulation::highest_set_bit as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_positive_number", source::tests::test_positive_number as fn()),
        ("test_0", source::tests::test_0 as fn()),
        ("test_1", source::tests::test_1 as fn()),
        ("test_2", source::tests::test_2 as fn()),
        ("test_3", source::tests::test_3 as fn()),
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
