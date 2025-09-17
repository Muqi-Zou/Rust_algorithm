#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::baby_step_giant_step as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("small_numbers", source::tests::small_numbers as fn()),
        ("primitive_root_tests", source::tests::primitive_root_tests as fn()),
        ("random_numbers", source::tests::random_numbers as fn()),
        ("no_solution", source::tests::no_solution as fn()),
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
