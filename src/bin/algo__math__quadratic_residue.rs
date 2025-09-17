#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::quadratic_residue as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("cipolla_small_numbers", source::tests::cipolla_small_numbers as fn()),
        ("tonelli_shanks_small_numbers", source::tests::tonelli_shanks_small_numbers as fn()),
        ("cipolla_random_numbers", source::tests::cipolla_random_numbers as fn()),
        ("tonelli_shanks_random_numbers", source::tests::tonelli_shanks_random_numbers as fn()),
        ("no_answer", source::tests::no_answer as fn()),
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
