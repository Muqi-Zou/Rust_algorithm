#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::rod_cutting as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_empty_prices", source::tests::test_empty_prices as fn()),
        ("test_example_with_three_prices", source::tests::test_example_with_three_prices as fn()),
        ("test_example_with_four_prices", source::tests::test_example_with_four_prices as fn()),
        ("test_example_with_five_prices", source::tests::test_example_with_five_prices as fn()),
        ("test_all_zeros_except_last", source::tests::test_all_zeros_except_last as fn()),
        ("test_descending_prices", source::tests::test_descending_prices as fn()),
        ("test_varied_prices", source::tests::test_varied_prices as fn()),
        ("test_complex_prices", source::tests::test_complex_prices as fn()),
        ("test_increasing_prices", source::tests::test_increasing_prices as fn()),
        ("test_large_range_prices", source::tests::test_large_range_prices as fn()),
        ("test_single_length_price", source::tests::test_single_length_price as fn()),
        ("test_zero_length_price", source::tests::test_zero_length_price as fn()),
        ("test_repeated_prices", source::tests::test_repeated_prices as fn()),
        ("test_no_profit", source::tests::test_no_profit as fn()),
        ("test_large_input", source::tests::test_large_input as fn()),
        ("test_all_zero_input", source::tests::test_all_zero_input as fn()),
        ("test_very_large_prices", source::tests::test_very_large_prices as fn()),
        ("test_greedy_does_not_work", source::tests::test_greedy_does_not_work as fn()),
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
