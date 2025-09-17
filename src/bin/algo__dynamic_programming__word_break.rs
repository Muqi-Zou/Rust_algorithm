#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::word_break as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("typical_case_1", source::tests::typical_case_1 as fn()),
        ("typical_case_2", source::tests::typical_case_2 as fn()),
        ("typical_case_3", source::tests::typical_case_3 as fn()),
        ("edge_case_empty_string", source::tests::edge_case_empty_string as fn()),
        ("edge_case_empty_dict", source::tests::edge_case_empty_dict as fn()),
        ("edge_case_single_char_in_dict", source::tests::edge_case_single_char_in_dict as fn()),
        ("edge_case_single_char_not_in_dict", source::tests::edge_case_single_char_not_in_dict as fn()),
        ("edge_case_all_words_larger_than_input", source::tests::edge_case_all_words_larger_than_input as fn()),
        ("edge_case_no_solution_large_string", source::tests::edge_case_no_solution_large_string as fn()),
        ("successful_segmentation_large_string", source::tests::successful_segmentation_large_string as fn()),
        ("long_string_repeated_pattern", source::tests::long_string_repeated_pattern as fn()),
        ("long_string_no_solution", source::tests::long_string_no_solution as fn()),
        ("mixed_size_dict_1", source::tests::mixed_size_dict_1 as fn()),
        ("mixed_size_dict_2", source::tests::mixed_size_dict_2 as fn()),
        ("mixed_size_dict_3", source::tests::mixed_size_dict_3 as fn()),
        ("performance_stress_test_large_valid", source::tests::performance_stress_test_large_valid as fn()),
        ("performance_stress_test_large_invalid", source::tests::performance_stress_test_large_invalid as fn()),
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
