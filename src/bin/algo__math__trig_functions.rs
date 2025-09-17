#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::trig_functions as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_sine", source::tests::test_sine as fn()),
        ("test_sine_bad_arg", source::tests::test_sine_bad_arg as fn()),
        ("test_cosine_bad_arg", source::tests::test_cosine_bad_arg as fn()),
        ("test_cosine", source::tests::test_cosine as fn()),
        ("test_tan_bad_arg", source::tests::test_tan_bad_arg as fn()),
        ("test_tan", source::tests::test_tan as fn()),
        ("test_cotan_bad_arg", source::tests::test_cotan_bad_arg as fn()),
        ("test_cotan", source::tests::test_cotan as fn()),
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
