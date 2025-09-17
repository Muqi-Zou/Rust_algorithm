#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::general::convex_hull as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("empty", source::tests::empty as fn()),
        ("not_enough_points", source::tests::not_enough_points as fn()),
        ("not_enough_points1", source::tests::not_enough_points1 as fn()),
        ("not_enough_points2", source::tests::not_enough_points2 as fn()),
        ("lots_of_points", source::tests::lots_of_points as fn()),
        ("lots_of_points2", source::tests::lots_of_points2 as fn()),
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
