#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::vector_cross_product as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_cross_product_and_magnitude_1", source::tests::test_cross_product_and_magnitude_1 as fn()),
        ("test_cross_product_and_magnitude_2", source::tests::test_cross_product_and_magnitude_2 as fn()),
        ("test_cross_product_and_magnitude_3", source::tests::test_cross_product_and_magnitude_3 as fn()),
        ("test_cross_product_and_magnitude_4", source::tests::test_cross_product_and_magnitude_4 as fn()),
        ("test_cross_product_and_magnitude_5", source::tests::test_cross_product_and_magnitude_5 as fn()),
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
