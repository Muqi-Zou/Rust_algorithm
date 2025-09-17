#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::lipogram as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("perfect_pangram", source::tests::perfect_pangram as fn()),
        ("HashSet", source::tests::HashSet as fn()),
        ("lipogram_single_missing", source::tests::lipogram_single_missing as fn()),
        ("HashSet", source::tests::HashSet as fn()),
        ("lipogram_multiple_missing", source::tests::lipogram_multiple_missing as fn()),
        ("HashSet", source::tests::HashSet as fn()),
        ("long_lipogram_single_missing", source::tests::long_lipogram_single_missing as fn()),
        ("HashSet", source::tests::HashSet as fn()),
        ("invalid_non_lowercase_chars", source::tests::invalid_non_lowercase_chars as fn()),
        ("HashSet", source::tests::HashSet as fn()),
        ("invalid_non_alphabetic_input", source::tests::invalid_non_alphabetic_input as fn()),
        ("HashSet", source::tests::HashSet as fn()),
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
