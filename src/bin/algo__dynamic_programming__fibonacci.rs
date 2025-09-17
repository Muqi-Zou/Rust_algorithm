#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::fibonacci as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_fibonacci", source::tests::test_fibonacci as fn()),
        ("test_recursive_fibonacci", source::tests::test_recursive_fibonacci as fn()),
        ("test_classical_fibonacci", source::tests::test_classical_fibonacci as fn()),
        ("test_logarithmic_fibonacci", source::tests::test_logarithmic_fibonacci as fn()),
        ("test_iterative_and_recursive_equivalence", source::tests::test_iterative_and_recursive_equivalence as fn()),
        ("test_classical_and_combinatorial_are_off_by_one", source::tests::test_classical_and_combinatorial_are_off_by_one as fn()),
        ("test_memoized_fibonacci", source::tests::test_memoized_fibonacci as fn()),
        ("test_matrix_fibonacci", source::tests::test_matrix_fibonacci as fn()),
        ("test_binary_lifting_fibonacci", source::tests::test_binary_lifting_fibonacci as fn()),
        ("test_nth_fibonacci_number_modulo_m", source::tests::test_nth_fibonacci_number_modulo_m as fn()),
        ("test_last_digit_of_the_sum_of_nth_fibonacci_number", source::tests::test_last_digit_of_the_sum_of_nth_fibonacci_number as fn()),
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
