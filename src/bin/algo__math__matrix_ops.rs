#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::matrix_ops as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_invalid_matrix", source::tests::test_invalid_matrix as fn()),
        ("test_empty_matrix", source::tests::test_empty_matrix as fn()),
        ("test_zero_matrix", source::tests::test_zero_matrix as fn()),
        ("test_identity_matrix", source::tests::test_identity_matrix as fn()),
        ("test_invalid_add", source::tests::test_invalid_add as fn()),
        ("test_add_i32", source::tests::test_add_i32 as fn()),
        ("test_add_f64", source::tests::test_add_f64 as fn()),
        ("test_invalid_sub", source::tests::test_invalid_sub as fn()),
        ("test_subtract_i32", source::tests::test_subtract_i32 as fn()),
        ("test_subtract_f64", source::tests::test_subtract_f64 as fn()),
        ("test_invalid_mul", source::tests::test_invalid_mul as fn()),
        ("test_mul_i32", source::tests::test_mul_i32 as fn()),
        ("test_mul_f64", source::tests::test_mul_f64 as fn()),
        ("test_transpose_i32", source::tests::test_transpose_i32 as fn()),
        ("test_transpose_f64", source::tests::test_transpose_f64 as fn()),
        ("test_matrix_scalar_zero_mul", source::tests::test_matrix_scalar_zero_mul as fn()),
        ("test_matrix_scalar_mul_i32", source::tests::test_matrix_scalar_mul_i32 as fn()),
        ("test_matrix_scalar_mul_f64", source::tests::test_matrix_scalar_mul_f64 as fn()),
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
