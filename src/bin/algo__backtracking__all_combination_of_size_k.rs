#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::backtracking::all_combination_of_size_k as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_generate_4_2", source::tests::test_generate_4_2 as fn()),
        ("test_generate_4_3", source::tests::test_generate_4_3 as fn()),
        ("test_generate_5_3", source::tests::test_generate_5_3 as fn()),
        ("test_generate_5_1", source::tests::test_generate_5_1 as fn()),
        ("test_empty", source::tests::test_empty as fn()),
        ("test_generate_n_eq_k", source::tests::test_generate_n_eq_k as fn()),
        ("test_generate_k_greater_than_n", source::tests::test_generate_k_greater_than_n as fn()),
        ("test_zero_range_with_nonzero_k", source::tests::test_zero_range_with_nonzero_k as fn()),
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
