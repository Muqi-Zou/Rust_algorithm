#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::linear_sieve as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("small_primes_list", source::tests::small_primes_list as fn()),
        ("divisible_by_mpf", source::tests::divisible_by_mpf as fn()),
        ("check_factorization", source::tests::check_factorization as fn()),
        ("check_number_of_primes", source::tests::check_number_of_primes as fn()),
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
