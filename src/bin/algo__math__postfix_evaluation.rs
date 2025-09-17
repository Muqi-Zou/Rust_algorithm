#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::postfix_evaluation as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_addition_of_two_numbers", source::tests::test_addition_of_two_numbers as fn()),
        ("test_multiplication_and_addition", source::tests::test_multiplication_and_addition as fn()),
        ("test_simple_division", source::tests::test_simple_division as fn()),
        ("test_operator_without_operands", source::tests::test_operator_without_operands as fn()),
        ("test_division_by_zero_error", source::tests::test_division_by_zero_error as fn()),
        ("test_invalid_operator_in_expression", source::tests::test_invalid_operator_in_expression as fn()),
        ("test_missing_operator_for_expression", source::tests::test_missing_operator_for_expression as fn()),
        ("test_extra_operands_in_expression", source::tests::test_extra_operands_in_expression as fn()),
        ("test_empty_expression_error", source::tests::test_empty_expression_error as fn()),
        ("test_single_number_expression", source::tests::test_single_number_expression as fn()),
        ("test_addition_of_negative_numbers", source::tests::test_addition_of_negative_numbers as fn()),
        ("test_complex_expression_with_multiplication_and_addition", source::tests::test_complex_expression_with_multiplication_and_addition as fn()),
        ("test_expression_with_extra_whitespace", source::tests::test_expression_with_extra_whitespace as fn()),
        ("test_valid_then_invalid_operator", source::tests::test_valid_then_invalid_operator as fn()),
        ("test_first_division_by_zero", source::tests::test_first_division_by_zero as fn()),
        ("test_complex_expression_with_multiple_operators", source::tests::test_complex_expression_with_multiple_operators as fn()),
        ("test_expression_with_only_whitespace", source::tests::test_expression_with_only_whitespace as fn()),
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
