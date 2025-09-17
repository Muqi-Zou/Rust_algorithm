#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::knapsack as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_basic_knapsack_small", source::tests::test_basic_knapsack_small as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
        ("test_basic_knapsack_tiny", source::tests::test_basic_knapsack_tiny as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
        ("test_basic_knapsack_medium", source::tests::test_basic_knapsack_medium as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
        ("test_diverse_weights_values_small", source::tests::test_diverse_weights_values_small as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
        ("test_diverse_weights_values_medium", source::tests::test_diverse_weights_values_medium as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
        ("test_high_value_items", source::tests::test_high_value_items as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
        ("test_large_knapsack", source::tests::test_large_knapsack as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
        ("test_zero_capacity", source::tests::test_zero_capacity as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
        ("test_very_small_capacity", source::tests::test_very_small_capacity as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
        ("test_no_items", source::tests::test_no_items as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
        ("test_item_too_heavy", source::tests::test_item_too_heavy as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
        ("test_greedy_algorithm_does_not_work", source::tests::test_greedy_algorithm_does_not_work as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
        ("test_greedy_algorithm_does_not_work_weight_smaller_than_capacity", source::tests::test_greedy_algorithm_does_not_work_weight_smaller_than_capacity as fn()),
        ("optimal_profit", source::tests::optimal_profit as fn()),
        ("total_weight", source::tests::total_weight as fn()),
        ("item_indices", source::tests::item_indices as fn()),
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
