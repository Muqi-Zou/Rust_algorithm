#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::geometry::graham_scan as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("too_few_points", source::tests::too_few_points as fn()),
        ("duplicate_point", source::tests::duplicate_point as fn()),
        ("points_same_line", source::tests::points_same_line as fn()),
        ("triangle", source::tests::triangle as fn()),
        ("rectangle", source::tests::rectangle as fn()),
        ("triangle_with_points_in_middle", source::tests::triangle_with_points_in_middle as fn()),
        ("rectangle_with_points_in_middle", source::tests::rectangle_with_points_in_middle as fn()),
        ("star", source::tests::star as fn()),
        ("rectangle_with_points_on_same_line", source::tests::rectangle_with_points_on_same_line as fn()),
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
