#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::simpsons_integration as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_simpsons_integration", source::tests::test_simpsons_integration as fn()),
        ("test_error", source::tests::test_error as fn()),
        ("test_convergence", source::tests::test_convergence as fn()),
        ("test_negative", source::tests::test_negative as fn()),
        ("test_non_zero_lower_bound", source::tests::test_non_zero_lower_bound as fn()),
        ("test_non_zero_upper_bound", source::tests::test_non_zero_upper_bound as fn()),
        ("test_non_zero_lower_and_upper_bound", source::tests::test_non_zero_lower_and_upper_bound as fn()),
        ("test_non_zero_lower_and_upper_bound_negative", source::tests::test_non_zero_lower_and_upper_bound_negative as fn()),
        ("parabola_curve_length", source::tests::parabola_curve_length as fn()),
        ("area_under_cosine", source::tests::area_under_cosine as fn()),
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
