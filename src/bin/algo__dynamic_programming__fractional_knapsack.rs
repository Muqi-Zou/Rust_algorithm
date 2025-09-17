#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::fractional_knapsack as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test", source::tests::test as fn()),
        ("test2", source::tests::test2 as fn()),
        ("test3", source::tests::test3 as fn()),
        ("test4", source::tests::test4 as fn()),
        ("test5", source::tests::test5 as fn()),
        ("test6", source::tests::test6 as fn()),
        ("test_nan", source::tests::test_nan as fn()),
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
