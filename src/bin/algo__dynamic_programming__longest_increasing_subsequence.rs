#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::longest_increasing_subsequence as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_empty_vec", source::tests::test_empty_vec as fn()),
        ("test_example_1", source::tests::test_example_1 as fn()),
        ("test_example_2", source::tests::test_example_2 as fn()),
        ("test_example_3", source::tests::test_example_3 as fn()),
        ("test_tle", source::tests::test_tle as fn()),
        ("test_negative_elements", source::tests::test_negative_elements as fn()),
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
