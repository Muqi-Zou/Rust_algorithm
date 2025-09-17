#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::trapezoidal_integration as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("basic_0", source::tests::basic_0 as fn()),
        ("basic_0_higher_prec", source::tests::basic_0_higher_prec as fn()),
        ("basic_1", source::tests::basic_1 as fn()),
        ("basic_1_higher_prec", source::tests::basic_1_higher_prec as fn()),
        ("flipped_limits", source::tests::flipped_limits as fn()),
        ("empty_range", source::tests::empty_range as fn()),
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
