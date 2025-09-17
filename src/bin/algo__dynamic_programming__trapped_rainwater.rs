#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::trapped_rainwater as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_trapped_rainwater_basic", source::tests::test_trapped_rainwater_basic as fn()),
        ("test_trapped_rainwater_peak_under_water", source::tests::test_trapped_rainwater_peak_under_water as fn()),
        ("test_bucket", source::tests::test_bucket as fn()),
        ("test_skewed_bucket", source::tests::test_skewed_bucket as fn()),
        ("test_trapped_rainwater_empty", source::tests::test_trapped_rainwater_empty as fn()),
        ("test_trapped_rainwater_flat", source::tests::test_trapped_rainwater_flat as fn()),
        ("test_trapped_rainwater_no_trapped_water", source::tests::test_trapped_rainwater_no_trapped_water as fn()),
        ("test_trapped_rainwater_single_elevation_map", source::tests::test_trapped_rainwater_single_elevation_map as fn()),
        ("test_trapped_rainwater_two_point_elevation_map", source::tests::test_trapped_rainwater_two_point_elevation_map as fn()),
        ("test_trapped_rainwater_large_elevation_map_difference", source::tests::test_trapped_rainwater_large_elevation_map_difference as fn()),
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
