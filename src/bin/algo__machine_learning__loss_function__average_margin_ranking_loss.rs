#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::machine_learning::loss_function::average_margin_ranking_loss as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("invalid_length0", source::tests::invalid_length0 as fn()),
        ("invalid_length1", source::tests::invalid_length1 as fn()),
        ("invalid_length2", source::tests::invalid_length2 as fn()),
        ("invalid_length3", source::tests::invalid_length3 as fn()),
        ("invalid_values", source::tests::invalid_values as fn()),
        ("invalid_y_true", source::tests::invalid_y_true as fn()),
        ("empty_inputs", source::tests::empty_inputs as fn()),
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
