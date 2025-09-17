#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::least_square_approx as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("ten_points_1st_degree", source::tests::ten_points_1st_degree as fn()),
        ("eight_points_5th_degree", source::tests::eight_points_5th_degree as fn()),
        ("four_points_2nd_degree", source::tests::four_points_2nd_degree as fn()),
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
