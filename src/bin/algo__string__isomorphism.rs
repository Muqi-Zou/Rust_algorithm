#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::isomorphism as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("isomorphic", source::tests::isomorphic as fn()),
        ("isomorphic_long", source::tests::isomorphic_long as fn()),
        ("not_isomorphic", source::tests::not_isomorphic as fn()),
        ("non_isomorphic_long", source::tests::non_isomorphic_long as fn()),
        ("isomorphic_unicode", source::tests::isomorphic_unicode as fn()),
        ("isomorphic_unicode_different_byte_size", source::tests::isomorphic_unicode_different_byte_size as fn()),
        ("empty", source::tests::empty as fn()),
        ("different_length", source::tests::different_length as fn()),
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
