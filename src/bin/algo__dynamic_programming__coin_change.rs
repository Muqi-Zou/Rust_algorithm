#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::coin_change as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_basic_case", source::tests::test_basic_case as fn()),
        ("test_multiple_denominations", source::tests::test_multiple_denominations as fn()),
        ("test_empty_coins", source::tests::test_empty_coins as fn()),
        ("test_zero_amount", source::tests::test_zero_amount as fn()),
        ("test_no_solution_small_coin", source::tests::test_no_solution_small_coin as fn()),
        ("test_no_solution_large_coin", source::tests::test_no_solution_large_coin as fn()),
        ("test_single_coin_large_amount", source::tests::test_single_coin_large_amount as fn()),
        ("test_large_amount_multiple_coins", source::tests::test_large_amount_multiple_coins as fn()),
        ("test_no_combination_possible", source::tests::test_no_combination_possible as fn()),
        ("test_exact_combination", source::tests::test_exact_combination as fn()),
        ("test_large_denomination_multiple_coins", source::tests::test_large_denomination_multiple_coins as fn()),
        ("test_small_amount_not_possible", source::tests::test_small_amount_not_possible as fn()),
        ("test_non_divisible_amount", source::tests::test_non_divisible_amount as fn()),
        ("test_all_multiples", source::tests::test_all_multiples as fn()),
        ("test_large_amount_mixed_coins", source::tests::test_large_amount_mixed_coins as fn()),
        ("test_prime_coins_and_amount", source::tests::test_prime_coins_and_amount as fn()),
        ("test_coins_larger_than_amount", source::tests::test_coins_larger_than_amount as fn()),
        ("test_repeating_denominations", source::tests::test_repeating_denominations as fn()),
        ("test_non_standard_denominations", source::tests::test_non_standard_denominations as fn()),
        ("test_very_large_denominations", source::tests::test_very_large_denominations as fn()),
        ("test_large_amount_performance", source::tests::test_large_amount_performance as fn()),
        ("test_powers_of_two", source::tests::test_powers_of_two as fn()),
        ("test_fibonacci_sequence", source::tests::test_fibonacci_sequence as fn()),
        ("test_mixed_small_large", source::tests::test_mixed_small_large as fn()),
        ("test_impossible_combinations", source::tests::test_impossible_combinations as fn()),
        ("test_greedy_approach_does_not_work", source::tests::test_greedy_approach_does_not_work as fn()),
        ("test_zero_denominations_no_solution", source::tests::test_zero_denominations_no_solution as fn()),
        ("test_zero_denominations_solution", source::tests::test_zero_denominations_solution as fn()),
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
