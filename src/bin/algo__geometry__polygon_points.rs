#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::geometry::polygon_points as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_calculate_cross", source::tests::test_calculate_cross as fn()),
        ("test_polygon_3_coordinates", source::tests::test_polygon_3_coordinates as fn()),
        ("test_polygon_4_coordinates", source::tests::test_polygon_4_coordinates as fn()),
        ("test_gcd_multiple_of_common_factor", source::tests::test_gcd_multiple_of_common_factor as fn()),
        ("test_boundary", source::tests::test_boundary as fn()),
        ("test_lattice_points", source::tests::test_lattice_points as fn()),
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
