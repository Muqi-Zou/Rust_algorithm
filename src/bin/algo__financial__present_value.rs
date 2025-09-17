#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::financial::present_value as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("general_inputs1", source::tests::general_inputs1 as fn()),
        ("general_inputs2", source::tests::general_inputs2 as fn()),
        ("general_inputs3", source::tests::general_inputs3 as fn()),
        ("zero_input", source::tests::zero_input as fn()),
        ("negative_discount_rate", source::tests::negative_discount_rate as fn()),
        ("empty_cash_flow", source::tests::empty_cash_flow as fn()),
        ("test1", source::tests::test1 as fn()),
        ("test2", source::tests::test2 as fn()),
        ("test3", source::tests::test3 as fn()),
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
