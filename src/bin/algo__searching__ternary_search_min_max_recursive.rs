#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::searching::ternary_search_min_max_recursive as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("finds_max_value", source::tests::finds_max_value as fn()),
        ("finds_min_value", source::tests::finds_min_value as fn()),
        ("finds_max_value_2", source::tests::finds_max_value_2 as fn()),
        ("finds_min_value_2", source::tests::finds_min_value_2 as fn()),
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
