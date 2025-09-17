#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::sorting::heap_sort as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("empty_array", source::tests::empty_array as fn()),
        ("single_element_array", source::tests::single_element_array as fn()),
        ("sorted", source::tests::sorted as fn()),
        ("sorted_desc", source::tests::sorted_desc as fn()),
        ("basic_0", source::tests::basic_0 as fn()),
        ("basic_1", source::tests::basic_1 as fn()),
        ("basic_2", source::tests::basic_2 as fn()),
        ("duplicated_elements", source::tests::duplicated_elements as fn()),
        ("strings", source::tests::strings as fn()),
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
