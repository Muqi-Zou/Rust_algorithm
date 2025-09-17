#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::ceil as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("positive_decimal", source::tests::positive_decimal as fn()),
        ("positive_decimal_with_small_number", source::tests::positive_decimal_with_small_number as fn()),
        ("positive_integer", source::tests::positive_integer as fn()),
        ("negative_decimal", source::tests::negative_decimal as fn()),
        ("negative_decimal_with_small_number", source::tests::negative_decimal_with_small_number as fn()),
        ("negative_integer", source::tests::negative_integer as fn()),
        ("zero", source::tests::zero as fn()),
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
