#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::bit_manipulation::n_bits_gray_code as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("zero_bit_count", source::tests::zero_bit_count as fn()),
        ("gray_code_1_bit", source::tests::gray_code_1_bit as fn()),
        ("gray_code_2_bit", source::tests::gray_code_2_bit as fn()),
        ("gray_code_3_bit", source::tests::gray_code_3_bit as fn()),
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
