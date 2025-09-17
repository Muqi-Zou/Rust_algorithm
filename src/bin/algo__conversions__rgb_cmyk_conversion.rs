#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::conversions::rgb_cmyk_conversion as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("white", source::tests::white as fn()),
        ("gray", source::tests::gray as fn()),
        ("black", source::tests::black as fn()),
        ("red", source::tests::red as fn()),
        ("green", source::tests::green as fn()),
        ("blue", source::tests::blue as fn()),
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
