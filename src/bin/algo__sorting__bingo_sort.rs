#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::sorting::bingo_sort as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_bingo_sort", source::tests::test_bingo_sort as fn()),
        ("test_empty_array", source::tests::test_empty_array as fn()),
        ("test_single_element_array", source::tests::test_single_element_array as fn()),
        ("test_negative_numbers", source::tests::test_negative_numbers as fn()),
        ("test_already_sorted", source::tests::test_already_sorted as fn()),
        ("test_reverse_sorted", source::tests::test_reverse_sorted as fn()),
        ("test_duplicates", source::tests::test_duplicates as fn()),
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
