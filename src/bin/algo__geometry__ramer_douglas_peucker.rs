#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::geometry::ramer_douglas_peucker as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("basic", source::tests::basic as fn()),
        ("basic_shifted_1", source::tests::basic_shifted_1 as fn()),
        ("basic_shifted_2", source::tests::basic_shifted_2 as fn()),
        ("test_ramer_douglas_peucker_polygon", source::tests::test_ramer_douglas_peucker_polygon as fn()),
        ("test_ramer_douglas_peucker_polygonal_chain", source::tests::test_ramer_douglas_peucker_polygonal_chain as fn()),
        ("test_less_than_three_points", source::tests::test_less_than_three_points as fn()),
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
