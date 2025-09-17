#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::aliquot_sum as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_with_1", source::tests::test_with_1 as fn()),
        ("test_with_2", source::tests::test_with_2 as fn()),
        ("test_with_3", source::tests::test_with_3 as fn()),
        ("test_with_4", source::tests::test_with_4 as fn()),
        ("test_with_5", source::tests::test_with_5 as fn()),
        ("test_with_6", source::tests::test_with_6 as fn()),
        ("test_with_7", source::tests::test_with_7 as fn()),
        ("test_with_8", source::tests::test_with_8 as fn()),
        ("test_with_9", source::tests::test_with_9 as fn()),
        ("test_with_10", source::tests::test_with_10 as fn()),
        ("test_with_15", source::tests::test_with_15 as fn()),
        ("test_with_343", source::tests::test_with_343 as fn()),
        ("test_with_344", source::tests::test_with_344 as fn()),
        ("test_with_500", source::tests::test_with_500 as fn()),
        ("test_with_501", source::tests::test_with_501 as fn()),
        ("panics_if_input_is_zero", source::tests::panics_if_input_is_zero as fn()),
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
