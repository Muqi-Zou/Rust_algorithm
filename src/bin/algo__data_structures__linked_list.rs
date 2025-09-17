#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::data_structures::linked_list as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("insert_at_tail_works", source::tests::insert_at_tail_works as fn()),
        ("insert_at_head_works", source::tests::insert_at_head_works as fn()),
        ("insert_at_ith_can_add_to_tail", source::tests::insert_at_ith_can_add_to_tail as fn()),
        ("insert_at_ith_can_add_to_head", source::tests::insert_at_ith_can_add_to_head as fn()),
        ("insert_at_ith_can_add_to_middle", source::tests::insert_at_ith_can_add_to_middle as fn()),
        ("insert_at_ith_and_delete_at_ith_in_the_middle", source::tests::insert_at_ith_and_delete_at_ith_in_the_middle as fn()),
        ("insert_at_ith_and_delete_ith_work_over_many_iterations", source::tests::insert_at_ith_and_delete_ith_work_over_many_iterations as fn()),
        ("delete_tail_works", source::tests::delete_tail_works as fn()),
        ("delete_head_works", source::tests::delete_head_works as fn()),
        ("delete_ith_can_delete_at_tail", source::tests::delete_ith_can_delete_at_tail as fn()),
        ("delete_ith_can_delete_at_head", source::tests::delete_ith_can_delete_at_head as fn()),
        ("delete_ith_can_delete_in_middle", source::tests::delete_ith_can_delete_in_middle as fn()),
        ("create_numeric_list", source::tests::create_numeric_list as fn()),
        ("create_string_list", source::tests::create_string_list as fn()),
        ("get_by_index_in_numeric_list", source::tests::get_by_index_in_numeric_list as fn()),
        ("get_by_index_in_string_list", source::tests::get_by_index_in_string_list as fn()),
        ("delete_ith_panics_if_index_equals_length", source::tests::delete_ith_panics_if_index_equals_length as fn()),
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
