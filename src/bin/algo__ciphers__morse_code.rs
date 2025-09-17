#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::ciphers::morse_code as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("encrypt_only_letters", source::tests::encrypt_only_letters as fn()),
        ("encrypt_letters_and_special_characters", source::tests::encrypt_letters_and_special_characters as fn()),
        ("encrypt_message_with_unsupported_character", source::tests::encrypt_message_with_unsupported_character as fn()),
        ("decrypt_valid_morsecode_with_spaces", source::tests::decrypt_valid_morsecode_with_spaces as fn()),
        ("decrypt_valid_character_set_invalid_morsecode", source::tests::decrypt_valid_character_set_invalid_morsecode as fn()),
        ("decrypt_invalid_morsecode_with_spaces", source::tests::decrypt_invalid_morsecode_with_spaces as fn()),
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
