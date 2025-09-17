#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::searching::saddleback_search as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_element_not_found", source::tests::test_element_not_found as fn()),
        ("test_element_at_top_left", source::tests::test_element_at_top_left as fn()),
        ("test_element_at_bottom_right", source::tests::test_element_at_bottom_right as fn()),
        ("test_element_at_top_right", source::tests::test_element_at_top_right as fn()),
        ("test_element_at_bottom_left", source::tests::test_element_at_bottom_left as fn()),
        ("test_element_in_middle", source::tests::test_element_in_middle as fn()),
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
