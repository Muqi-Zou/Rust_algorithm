#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::matrix_chain_multiply as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("basic_chain_of_matrices", source::tests::basic_chain_of_matrices as fn()),
        ("chain_of_large_matrices", source::tests::chain_of_large_matrices as fn()),
        ("long_chain_of_matrices", source::tests::long_chain_of_matrices as fn()),
        ("complex_chain_of_matrices", source::tests::complex_chain_of_matrices as fn()),
        ("empty_dimensions_input", source::tests::empty_dimensions_input as fn()),
        ("single_dimensions_input", source::tests::single_dimensions_input as fn()),
        ("single_matrix_input", source::tests::single_matrix_input as fn()),
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
