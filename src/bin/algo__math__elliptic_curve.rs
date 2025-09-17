#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::elliptic_curve as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_char_2_panic", source::tests::test_char_2_panic as fn()),
        ("test_char_3_panic", source::tests::test_char_3_panic as fn()),
        ("test_singular_panic", source::tests::test_singular_panic as fn()),
        ("e_5_1_0_group_table", source::tests::e_5_1_0_group_table as fn()),
        ("group_law", source::tests::group_law as fn()),
        ("cardinality", source::tests::cardinality as fn()),
        ("cardinality_perf", source::tests::cardinality_perf as fn()),
        ("cardinality_large_prime", source::tests::cardinality_large_prime as fn()),
        ("test_points", source::tests::test_points as fn()),
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
