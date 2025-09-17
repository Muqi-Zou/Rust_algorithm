#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::bit_manipulation::counting_bits as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_count_set_bits_zero", source::tests::test_count_set_bits_zero as fn()),
        ("test_count_set_bits_one", source::tests::test_count_set_bits_one as fn()),
        ("test_count_set_bits_power_of_two", source::tests::test_count_set_bits_power_of_two as fn()),
        ("test_count_set_bits_all_set_bits", source::tests::test_count_set_bits_all_set_bits as fn()),
        ("test_count_set_bits_alternating_bits", source::tests::test_count_set_bits_alternating_bits as fn()),
        ("test_count_set_bits_mixed_bits", source::tests::test_count_set_bits_mixed_bits as fn()),
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
