#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::sorting::quick_sort as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("basic", source::tests::basic as fn()),
        ("basic_string", source::tests::basic_string as fn()),
        ("empty", source::tests::empty as fn()),
        ("one_element", source::tests::one_element as fn()),
        ("pre_sorted", source::tests::pre_sorted as fn()),
        ("reverse_sorted", source::tests::reverse_sorted as fn()),
        ("large_elements", source::tests::large_elements as fn()),
        ("nearly_ordered_elements", source::tests::nearly_ordered_elements as fn()),
        ("repeated_elements", source::tests::repeated_elements as fn()),
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
