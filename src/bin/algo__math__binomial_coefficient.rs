#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::binomial_coefficient as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_binom_5_2", source::tests::test_binom_5_2 as fn()),
        ("test_binom_10_5", source::tests::test_binom_10_5 as fn()),
        ("test_binom_0_0", source::tests::test_binom_0_0 as fn()),
        ("test_binom_large_n_small_k", source::tests::test_binom_large_n_small_k as fn()),
        ("test_binom_random_1", source::tests::test_binom_random_1 as fn()),
        ("test_binom_random_2", source::tests::test_binom_random_2 as fn()),
        ("test_binom_random_3", source::tests::test_binom_random_3 as fn()),
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
