#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::data_structures::range_minimum_query as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("small", source::tests::small as fn()),
        ("medium", source::tests::medium as fn()),
        ("large", source::tests::large as fn()),
        ("simple_query_tests", source::tests::simple_query_tests as fn()),
        ("float_query_tests", source::tests::float_query_tests as fn()),
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
