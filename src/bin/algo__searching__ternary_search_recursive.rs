#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::searching::ternary_search_recursive as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("returns_none_if_empty_list", source::tests::returns_none_if_empty_list as fn()),
        ("returns_none_if_range_is_invalid", source::tests::returns_none_if_range_is_invalid as fn()),
        ("returns_index_if_list_has_one_item", source::tests::returns_index_if_list_has_one_item as fn()),
        ("returns_first_index", source::tests::returns_first_index as fn()),
        ("returns_first_index_if_end_out_of_bounds", source::tests::returns_first_index_if_end_out_of_bounds as fn()),
        ("returns_last_index", source::tests::returns_last_index as fn()),
        ("returns_last_index_if_end_out_of_bounds", source::tests::returns_last_index_if_end_out_of_bounds as fn()),
        ("returns_middle_index", source::tests::returns_middle_index as fn()),
        ("returns_middle_index_if_end_out_of_bounds", source::tests::returns_middle_index_if_end_out_of_bounds as fn()),
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
