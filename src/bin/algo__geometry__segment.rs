#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::geometry::segment as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("colinear", source::tests::colinear as fn()),
        ("colinear_vertical", source::tests::colinear_vertical as fn()),
        ("intersects", source::tests::intersects as fn()),
        ("intersects_endpoint_on_segment", source::tests::intersects_endpoint_on_segment as fn()),
        ("intersects_self", source::tests::intersects_self as fn()),
        ("too_short_to_intersect", source::tests::too_short_to_intersect as fn()),
        ("parallel_segments", source::tests::parallel_segments as fn()),
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
