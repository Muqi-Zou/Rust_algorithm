#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::graph::decremental_connectivity as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("construction_test", source::tests::construction_test as fn()),
        ("non_bidirectional_test", source::tests::non_bidirectional_test as fn()),
        ("delete_panic_test", source::tests::delete_panic_test as fn()),
        ("query_test", source::tests::query_test as fn()),
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
