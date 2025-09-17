#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::hamming_distance as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("empty_inputs", source::tests::empty_inputs as fn()),
        ("different_length", source::tests::different_length as fn()),
        ("length_1_inputs_identical", source::tests::length_1_inputs_identical as fn()),
        ("length_1_inputs_different", source::tests::length_1_inputs_different as fn()),
        ("same_strings", source::tests::same_strings as fn()),
        ("regular_input_0", source::tests::regular_input_0 as fn()),
        ("regular_input_1", source::tests::regular_input_1 as fn()),
        ("regular_input_2", source::tests::regular_input_2 as fn()),
        ("different_case", source::tests::different_case as fn()),
        ("strings_with_no_common_chars", source::tests::strings_with_no_common_chars as fn()),
        ("long_strings_one_diff", source::tests::long_strings_one_diff as fn()),
        ("long_strings_many_diffs", source::tests::long_strings_many_diffs as fn()),
        ("strings_with_special_chars_identical", source::tests::strings_with_special_chars_identical as fn()),
        ("strings_with_special_chars_diff", source::tests::strings_with_special_chars_diff as fn()),
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
