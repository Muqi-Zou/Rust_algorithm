#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::searching::binary_search_recursive as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("empty", source::tests::empty as fn()),
        ("one_item_found", source::tests::one_item_found as fn()),
        ("one_item_not_found", source::tests::one_item_not_found as fn()),
        ("search_strings_asc_start", source::tests::search_strings_asc_start as fn()),
        ("search_strings_asc_middle", source::tests::search_strings_asc_middle as fn()),
        ("search_strings_asc_last", source::tests::search_strings_asc_last as fn()),
        ("search_strings_asc_not_found", source::tests::search_strings_asc_not_found as fn()),
        ("search_strings_desc_start", source::tests::search_strings_desc_start as fn()),
        ("search_strings_desc_middle", source::tests::search_strings_desc_middle as fn()),
        ("search_strings_desc_last", source::tests::search_strings_desc_last as fn()),
        ("search_strings_desc_not_found", source::tests::search_strings_desc_not_found as fn()),
        ("search_ints_asc_start", source::tests::search_ints_asc_start as fn()),
        ("search_ints_asc_middle", source::tests::search_ints_asc_middle as fn()),
        ("search_ints_asc_end", source::tests::search_ints_asc_end as fn()),
        ("search_ints_asc_not_found", source::tests::search_ints_asc_not_found as fn()),
        ("search_ints_desc_start", source::tests::search_ints_desc_start as fn()),
        ("search_ints_desc_middle", source::tests::search_ints_desc_middle as fn()),
        ("search_ints_desc_end", source::tests::search_ints_desc_end as fn()),
        ("search_ints_desc_not_found", source::tests::search_ints_desc_not_found as fn()),
        ("with_gaps_0", source::tests::with_gaps_0 as fn()),
        ("with_gaps_1", source::tests::with_gaps_1 as fn()),
        ("with_gaps_2", source::tests::with_gaps_2 as fn()),
        ("with_gaps_3", source::tests::with_gaps_3 as fn()),
        ("with_gaps_4", source::tests::with_gaps_4 as fn()),
        ("with_gaps_5", source::tests::with_gaps_5 as fn()),
        ("with_gaps_6", source::tests::with_gaps_6 as fn()),
        ("with_gaps_7", source::tests::with_gaps_7 as fn()),
        ("with_gaps_8", source::tests::with_gaps_8 as fn()),
        ("with_gaps_9", source::tests::with_gaps_9 as fn()),
        ("with_gaps_10", source::tests::with_gaps_10 as fn()),
        ("with_gaps_11", source::tests::with_gaps_11 as fn()),
        ("with_gaps_12", source::tests::with_gaps_12 as fn()),
        ("with_gaps_13", source::tests::with_gaps_13 as fn()),
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
