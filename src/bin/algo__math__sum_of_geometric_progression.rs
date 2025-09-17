#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::sum_of_geometric_progression as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("regular_input_0", source::tests::regular_input_0 as fn()),
        ("regular_input_1", source::tests::regular_input_1 as fn()),
        ("regular_input_2", source::tests::regular_input_2 as fn()),
        ("common_ratio_one", source::tests::common_ratio_one as fn()),
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
