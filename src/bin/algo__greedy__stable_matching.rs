#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::greedy::stable_matching as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_stable_matching_scenario_1", source::tests::test_stable_matching_scenario_1 as fn()),
        ("test_stable_matching_empty", source::tests::test_stable_matching_empty as fn()),
        ("test_stable_matching_duplicate_preferences", source::tests::test_stable_matching_duplicate_preferences as fn()),
        ("test_stable_matching_single_pair", source::tests::test_stable_matching_single_pair as fn()),
        ("test_woman_prefers_new_man", source::tests::test_woman_prefers_new_man as fn()),
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
