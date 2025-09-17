#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::ciphers::caesar as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("alphabet_length_should_be_26", source::tests::alphabet_length_should_be_26 as fn()),
        ("empty_text", source::tests::empty_text as fn()),
        ("rot_13", source::tests::rot_13 as fn()),
        ("unicode", source::tests::unicode as fn()),
        ("rotation_within_alphabet_range", source::tests::rotation_within_alphabet_range as fn()),
        ("no_rotation", source::tests::no_rotation as fn()),
        ("rotation_at_alphabet_end", source::tests::rotation_at_alphabet_end as fn()),
        ("longer", source::tests::longer as fn()),
        ("non_alphabetic_characters", source::tests::non_alphabetic_characters as fn()),
        ("uppercase_letters", source::tests::uppercase_letters as fn()),
        ("mixed_case", source::tests::mixed_case as fn()),
        ("with_whitespace", source::tests::with_whitespace as fn()),
        ("with_special_characters", source::tests::with_special_characters as fn()),
        ("with_numbers", source::tests::with_numbers as fn()),
        ("negative_rotation", source::tests::negative_rotation as fn()),
        ("empty_input_negative_rotation", source::tests::empty_input_negative_rotation as fn()),
        ("empty_input_large_rotation", source::tests::empty_input_large_rotation as fn()),
        ("large_rotation", source::tests::large_rotation as fn()),
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
