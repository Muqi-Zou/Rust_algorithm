#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::lucas_series as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("input_0", source::tests::input_0 as fn()),
        ("input_1", source::tests::input_1 as fn()),
        ("input_2", source::tests::input_2 as fn()),
        ("input_3", source::tests::input_3 as fn()),
        ("input_4", source::tests::input_4 as fn()),
        ("input_5", source::tests::input_5 as fn()),
        ("input_6", source::tests::input_6 as fn()),
        ("input_7", source::tests::input_7 as fn()),
        ("input_8", source::tests::input_8 as fn()),
        ("input_9", source::tests::input_9 as fn()),
        ("input_10", source::tests::input_10 as fn()),
        ("input_15", source::tests::input_15 as fn()),
        ("input_20", source::tests::input_20 as fn()),
        ("input_25", source::tests::input_25 as fn()),
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
