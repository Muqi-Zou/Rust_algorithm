#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::rabin_karp as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("single_match_at_start", source::tests::single_match_at_start as fn()),
        ("single_match_at_end", source::tests::single_match_at_end as fn()),
        ("single_match_in_middle", source::tests::single_match_in_middle as fn()),
        ("multiple_matches", source::tests::multiple_matches as fn()),
        ("overlapping_matches", source::tests::overlapping_matches as fn()),
        ("no_match", source::tests::no_match as fn()),
        ("pattern_is_entire_string", source::tests::pattern_is_entire_string as fn()),
        ("target_is_multiple_patterns", source::tests::target_is_multiple_patterns as fn()),
        ("empty_text", source::tests::empty_text as fn()),
        ("empty_pattern", source::tests::empty_pattern as fn()),
        ("empty_text_and_pattern", source::tests::empty_text_and_pattern as fn()),
        ("pattern_larger_than_text", source::tests::pattern_larger_than_text as fn()),
        ("large_text_small_pattern", source::tests::large_text_small_pattern as fn()),
        ("single_char_match", source::tests::single_char_match as fn()),
        ("single_char_no_match", source::tests::single_char_no_match as fn()),
        ("large_pattern_no_match", source::tests::large_pattern_no_match as fn()),
        ("repeating_chars", source::tests::repeating_chars as fn()),
        ("special_characters", source::tests::special_characters as fn()),
        ("numeric_and_alphabetic_mix", source::tests::numeric_and_alphabetic_mix as fn()),
        ("case_sensitivity", source::tests::case_sensitivity as fn()),
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
