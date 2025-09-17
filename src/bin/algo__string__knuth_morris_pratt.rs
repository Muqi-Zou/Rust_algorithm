#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::knuth_morris_pratt as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("each_letter_matches", source::tests::each_letter_matches as fn()),
        ("a_few_seperate_matches", source::tests::a_few_seperate_matches as fn()),
        ("unicode", source::tests::unicode as fn()),
        ("unicode_no_match_but_similar_bytes", source::tests::unicode_no_match_but_similar_bytes as fn()),
        ("one_match", source::tests::one_match as fn()),
        ("lots_of_matches", source::tests::lots_of_matches as fn()),
        ("lots_of_intricate_matches", source::tests::lots_of_intricate_matches as fn()),
        ("not_found0", source::tests::not_found0 as fn()),
        ("not_found1", source::tests::not_found1 as fn()),
        ("not_found2", source::tests::not_found2 as fn()),
        ("empty_string", source::tests::empty_string as fn()),
        ("empty_pattern", source::tests::empty_pattern as fn()),
        ("single_character_string", source::tests::single_character_string as fn()),
        ("single_character_pattern", source::tests::single_character_pattern as fn()),
        ("pattern_at_start", source::tests::pattern_at_start as fn()),
        ("pattern_at_end", source::tests::pattern_at_end as fn()),
        ("pattern_in_middle", source::tests::pattern_in_middle as fn()),
        ("no_match_with_repeated_characters", source::tests::no_match_with_repeated_characters as fn()),
        ("pattern_longer_than_string", source::tests::pattern_longer_than_string as fn()),
        ("very_long_string", source::tests::very_long_string as fn()),
        ("very_long_pattern", source::tests::very_long_pattern as fn()),
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
