#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::backtracking::n_queens as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_0_queens", source::tests::test_0_queens as fn()),
        ("test_1_queen", source::tests::test_1_queen as fn()),
        ("test_2_queens", source::tests::test_2_queens as fn()),
        ("test_3_queens", source::tests::test_3_queens as fn()),
        ("test_4_queens", source::tests::test_4_queens as fn()),
        ("test_5_queens", source::tests::test_5_queens as fn()),
        ("test_6_queens", source::tests::test_6_queens as fn()),
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
