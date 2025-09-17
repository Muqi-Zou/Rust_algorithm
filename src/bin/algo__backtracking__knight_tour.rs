#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::backtracking::knight_tour as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_knight_tour_5x5", source::tests::test_knight_tour_5x5 as fn()),
        ("test_knight_tour_6x6", source::tests::test_knight_tour_6x6 as fn()),
        ("test_knight_tour_8x8", source::tests::test_knight_tour_8x8 as fn()),
        ("test_no_solution", source::tests::test_no_solution as fn()),
        ("test_invalid_start_position", source::tests::test_invalid_start_position as fn()),
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
