#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::geometry::closest_points as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("zero_points", source::tests::zero_points as fn()),
        ("one_points", source::tests::one_points as fn()),
        ("two_points", source::tests::two_points as fn()),
        ("three_points", source::tests::three_points as fn()),
        ("list_1", source::tests::list_1 as fn()),
        ("list_2", source::tests::list_2 as fn()),
        ("vertical_points", source::tests::vertical_points as fn()),
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
