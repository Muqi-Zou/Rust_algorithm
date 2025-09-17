#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::machine_learning::loss_function::negative_log_likelihood as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("different_length", source::tests::different_length as fn()),
        ("different_length_one_empty", source::tests::different_length_one_empty as fn()),
        ("value_greater_than_1", source::tests::value_greater_than_1 as fn()),
        ("value_greater_smaller_than_0", source::tests::value_greater_smaller_than_0 as fn()),
        ("empty_input", source::tests::empty_input as fn()),
        ("set_0", source::tests::set_0 as fn()),
        ("set_1", source::tests::set_1 as fn()),
        ("set_2", source::tests::set_2 as fn()),
        ("set_3", source::tests::set_3 as fn()),
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
