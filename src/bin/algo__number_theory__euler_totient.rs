#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::number_theory::euler_totient as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("prime_2", source::tests::prime_2 as fn()),
        ("prime_3", source::tests::prime_3 as fn()),
        ("prime_5", source::tests::prime_5 as fn()),
        ("prime_7", source::tests::prime_7 as fn()),
        ("prime_11", source::tests::prime_11 as fn()),
        ("prime_13", source::tests::prime_13 as fn()),
        ("prime_17", source::tests::prime_17 as fn()),
        ("prime_19", source::tests::prime_19 as fn()),
        ("composite_6", source::tests::composite_6 as fn()),
        ("composite_10", source::tests::composite_10 as fn()),
        ("composite_15", source::tests::composite_15 as fn()),
        ("composite_12", source::tests::composite_12 as fn()),
        ("composite_18", source::tests::composite_18 as fn()),
        ("composite_20", source::tests::composite_20 as fn()),
        ("composite_30", source::tests::composite_30 as fn()),
        ("prime_power_2_to_2", source::tests::prime_power_2_to_2 as fn()),
        ("prime_power_2_to_3", source::tests::prime_power_2_to_3 as fn()),
        ("prime_power_3_to_2", source::tests::prime_power_3_to_2 as fn()),
        ("prime_power_2_to_4", source::tests::prime_power_2_to_4 as fn()),
        ("prime_power_5_to_2", source::tests::prime_power_5_to_2 as fn()),
        ("prime_power_3_to_3", source::tests::prime_power_3_to_3 as fn()),
        ("prime_power_2_to_5", source::tests::prime_power_2_to_5 as fn()),
        ("large_50", source::tests::large_50 as fn()),
        ("large_100", source::tests::large_100 as fn()),
        ("large_1000", source::tests::large_1000 as fn()),
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
