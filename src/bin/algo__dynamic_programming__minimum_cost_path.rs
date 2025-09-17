#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::minimum_cost_path as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("basic", source::tests::basic as fn()),
        ("single_element", source::tests::single_element as fn()),
        ("single_row", source::tests::single_row as fn()),
        ("single_column", source::tests::single_column as fn()),
        ("large_matrix", source::tests::large_matrix as fn()),
        ("uniform_matrix", source::tests::uniform_matrix as fn()),
        ("increasing_values", source::tests::increasing_values as fn()),
        ("high_cost_path", source::tests::high_cost_path as fn()),
        ("complex_matrix", source::tests::complex_matrix as fn()),
        ("empty_matrix", source::tests::empty_matrix as fn()),
        ("empty_row", source::tests::empty_row as fn()),
        ("non_rectangular", source::tests::non_rectangular as fn()),
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
