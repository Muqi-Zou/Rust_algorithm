#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::perfect_cube as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("num_0_a_perfect_cube", source::tests::num_0_a_perfect_cube as fn()),
        ("num_1_is_a_perfect_cube", source::tests::num_1_is_a_perfect_cube as fn()),
        ("num_27_is_a_perfect_cube", source::tests::num_27_is_a_perfect_cube as fn()),
        ("num_64_is_a_perfect_cube", source::tests::num_64_is_a_perfect_cube as fn()),
        ("num_8_is_a_perfect_cube", source::tests::num_8_is_a_perfect_cube as fn()),
        ("num_2_is_not_a_perfect_cube", source::tests::num_2_is_not_a_perfect_cube as fn()),
        ("num_3_is_not_a_perfect_cube", source::tests::num_3_is_not_a_perfect_cube as fn()),
        ("num_4_is_not_a_perfect_cube", source::tests::num_4_is_not_a_perfect_cube as fn()),
        ("num_5_is_not_a_perfect_cube", source::tests::num_5_is_not_a_perfect_cube as fn()),
        ("num_999_is_not_a_perfect_cube", source::tests::num_999_is_not_a_perfect_cube as fn()),
        ("num_1000_is_a_perfect_cube", source::tests::num_1000_is_a_perfect_cube as fn()),
        ("num_1001_is_not_a_perfect_cube", source::tests::num_1001_is_not_a_perfect_cube as fn()),
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
