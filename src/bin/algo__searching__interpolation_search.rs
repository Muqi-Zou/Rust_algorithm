#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::searching::interpolation_search as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("returns_err_if_empty_slice", source::tests::returns_err_if_empty_slice as fn()),
        ("returns_err_if_target_not_found", source::tests::returns_err_if_target_not_found as fn()),
        ("returns_first_index", source::tests::returns_first_index as fn()),
        ("returns_last_index", source::tests::returns_last_index as fn()),
        ("returns_middle_index", source::tests::returns_middle_index as fn()),
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
