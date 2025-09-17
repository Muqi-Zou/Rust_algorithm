#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::sorting::insertion_sort as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("empty", source::tests::empty as fn()),
        ("one_element", source::tests::one_element as fn()),
        ("already_sorted", source::tests::already_sorted as fn()),
        ("basic", source::tests::basic as fn()),
        ("odd_number_of_elements", source::tests::odd_number_of_elements as fn()),
        ("repeated_elements", source::tests::repeated_elements as fn()),
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
