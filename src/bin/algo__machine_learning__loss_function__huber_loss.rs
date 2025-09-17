#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::machine_learning::loss_function::huber_loss as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_huber_loss_residual_less_than_delta", source::tests::test_huber_loss_residual_less_than_delta as fn()),
        ("test_huber_loss_residual_greater_than_delta", source::tests::test_huber_loss_residual_greater_than_delta as fn()),
        ("test_huber_loss_invalid_length", source::tests::test_huber_loss_invalid_length as fn()),
        ("test_huber_loss_empty_prediction", source::tests::test_huber_loss_empty_prediction as fn()),
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
