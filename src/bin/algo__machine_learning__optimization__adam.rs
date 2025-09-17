#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::machine_learning::optimization::adam as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_adam_init_default_values", source::tests::test_adam_init_default_values as fn()),
        ("test_adam_init_custom_lr_value", source::tests::test_adam_init_custom_lr_value as fn()),
        ("test_adam_init_custom_betas_value", source::tests::test_adam_init_custom_betas_value as fn()),
        ("test_adam_init_custom_epsilon_value", source::tests::test_adam_init_custom_epsilon_value as fn()),
        ("test_adam_init_all_custom_values", source::tests::test_adam_init_all_custom_values as fn()),
        ("test_adam_step_default_params", source::tests::test_adam_step_default_params as fn()),
        ("test_adam_step_custom_params", source::tests::test_adam_step_custom_params as fn()),
        ("test_adam_step_empty_gradients_array", source::tests::test_adam_step_empty_gradients_array as fn()),
        ("test_adam_step_iteratively_until_convergence_with_default_params", source::tests::test_adam_step_iteratively_until_convergence_with_default_params as fn()),
        ("test_adam_step_iteratively_until_convergence_with_custom_params", source::tests::test_adam_step_iteratively_until_convergence_with_custom_params as fn()),
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
