#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::greatest_common_divisor as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("positive_number_recursive", source::tests::positive_number_recursive as fn()),
        ("positive_number_iterative", source::tests::positive_number_iterative as fn()),
        ("positive_number_stein", source::tests::positive_number_stein as fn()),
        ("negative_number_recursive", source::tests::negative_number_recursive as fn()),
        ("negative_number_iterative", source::tests::negative_number_iterative as fn()),
        ("mix_recursive", source::tests::mix_recursive as fn()),
        ("mix_iterative", source::tests::mix_iterative as fn()),
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
