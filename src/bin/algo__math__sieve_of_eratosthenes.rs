#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::sieve_of_eratosthenes as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_0", source::tests::test_0 as fn()),
        ("test_1", source::tests::test_1 as fn()),
        ("test_2", source::tests::test_2 as fn()),
        ("test_3", source::tests::test_3 as fn()),
        ("test_4", source::tests::test_4 as fn()),
        ("test_5", source::tests::test_5 as fn()),
        ("test_6", source::tests::test_6 as fn()),
        ("test_7", source::tests::test_7 as fn()),
        ("test_11", source::tests::test_11 as fn()),
        ("test_23", source::tests::test_23 as fn()),
        ("test_24", source::tests::test_24 as fn()),
        ("test_25", source::tests::test_25 as fn()),
        ("test_26", source::tests::test_26 as fn()),
        ("test_27", source::tests::test_27 as fn()),
        ("test_28", source::tests::test_28 as fn()),
        ("test_29", source::tests::test_29 as fn()),
        ("test_33", source::tests::test_33 as fn()),
        ("test_100", source::tests::test_100 as fn()),
        ("test_997", source::tests::test_997 as fn()),
        ("test_998", source::tests::test_998 as fn()),
        ("test_999", source::tests::test_999 as fn()),
        ("test_1000", source::tests::test_1000 as fn()),
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
