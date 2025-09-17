#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::big_integer::multiply as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("multiply0", source::tests::multiply0 as fn()),
        ("multiply1", source::tests::multiply1 as fn()),
        ("multiply_zero", source::tests::multiply_zero as fn()),
        ("other_1", source::tests::other_1 as fn()),
        ("other_2", source::tests::other_2 as fn()),
        ("other_3", source::tests::other_3 as fn()),
        ("other_4", source::tests::other_4 as fn()),
        ("empty_input", source::tests::empty_input as fn()),
        ("leading_zero", source::tests::leading_zero as fn()),
        ("wrong_characters", source::tests::wrong_characters as fn()),
        ("wrong_input_and_zero_1", source::tests::wrong_input_and_zero_1 as fn()),
        ("wrong_input_and_zero_2", source::tests::wrong_input_and_zero_2 as fn()),
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
