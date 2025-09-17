#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::bit_manipulation::sum_of_two_integers as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_add_two_integers_positive", source::tests::test_add_two_integers_positive as fn()),
        ("test_add_two_integers_large_positive", source::tests::test_add_two_integers_large_positive as fn()),
        ("test_add_two_integers_edge_positive", source::tests::test_add_two_integers_edge_positive as fn()),
        ("test_add_two_integers_negative", source::tests::test_add_two_integers_negative as fn()),
        ("test_add_two_integers_both_negative", source::tests::test_add_two_integers_both_negative as fn()),
        ("test_add_two_integers_edge_negative", source::tests::test_add_two_integers_edge_negative as fn()),
        ("test_add_two_integers_zero", source::tests::test_add_two_integers_zero as fn()),
        ("test_add_two_integers_zero_with_positive", source::tests::test_add_two_integers_zero_with_positive as fn()),
        ("test_add_two_integers_zero_with_negative", source::tests::test_add_two_integers_zero_with_negative as fn()),
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
