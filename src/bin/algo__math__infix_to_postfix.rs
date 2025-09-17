#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::infix_to_postfix as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("single_symbol", source::tests::single_symbol as fn()),
        ("simple_sum", source::tests::simple_sum as fn()),
        ("multiply_sum_left", source::tests::multiply_sum_left as fn()),
        ("multiply_sum_right", source::tests::multiply_sum_right as fn()),
        ("multiply_two_sums", source::tests::multiply_two_sums as fn()),
        ("product_and_power", source::tests::product_and_power as fn()),
        ("power_and_product", source::tests::power_and_product as fn()),
        ("product_of_powers", source::tests::product_of_powers as fn()),
        ("product_in_exponent", source::tests::product_in_exponent as fn()),
        ("regular_0", source::tests::regular_0 as fn()),
        ("regular_1", source::tests::regular_1 as fn()),
        ("regular_2", source::tests::regular_2 as fn()),
        ("unknown_character", source::tests::unknown_character as fn()),
        ("unmatched_paren", source::tests::unmatched_paren as fn()),
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
