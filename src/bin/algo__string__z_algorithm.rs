#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::z_algorithm as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("simple_match", source::tests::simple_match as fn()),
        ("no_match", source::tests::no_match as fn()),
        ("single_char_match", source::tests::single_char_match as fn()),
        ("overlapping_match", source::tests::overlapping_match as fn()),
        ("full_string_match", source::tests::full_string_match as fn()),
        ("empty_pattern", source::tests::empty_pattern as fn()),
        ("pattern_larger_than_text", source::tests::pattern_larger_than_text as fn()),
        ("repeated_pattern_in_text", source::tests::repeated_pattern_in_text as fn()),
        ("pattern_not_in_lipsum", source::tests::pattern_not_in_lipsum as fn()),
        ("pattern_in_lipsum", source::tests::pattern_in_lipsum as fn()),
        ("basic_z_array", source::tests::basic_z_array as fn()),
        ("empty_string", source::tests::empty_string as fn()),
        ("single_char_z_array", source::tests::single_char_z_array as fn()),
        ("repeated_char_z_array", source::tests::repeated_char_z_array as fn()),
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
