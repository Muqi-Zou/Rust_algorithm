#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::egg_dropping as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_no_floors", source::tests::test_no_floors as fn()),
        ("test_one_egg_multiple_floors", source::tests::test_one_egg_multiple_floors as fn()),
        ("test_multiple_eggs_one_floor", source::tests::test_multiple_eggs_one_floor as fn()),
        ("test_two_eggs_two_floors", source::tests::test_two_eggs_two_floors as fn()),
        ("test_three_eggs_five_floors", source::tests::test_three_eggs_five_floors as fn()),
        ("test_two_eggs_ten_floors", source::tests::test_two_eggs_ten_floors as fn()),
        ("test_two_eggs_thirty_six_floors", source::tests::test_two_eggs_thirty_six_floors as fn()),
        ("test_many_eggs_one_floor", source::tests::test_many_eggs_one_floor as fn()),
        ("test_many_eggs_few_floors", source::tests::test_many_eggs_few_floors as fn()),
        ("test_few_eggs_many_floors", source::tests::test_few_eggs_many_floors as fn()),
        ("test_zero_eggs", source::tests::test_zero_eggs as fn()),
        ("test_no_eggs_no_floors", source::tests::test_no_eggs_no_floors as fn()),
        ("test_one_egg_no_floors", source::tests::test_one_egg_no_floors as fn()),
        ("test_one_egg_one_floor", source::tests::test_one_egg_one_floor as fn()),
        ("test_maximum_floors_one_egg", source::tests::test_maximum_floors_one_egg as fn()),
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
