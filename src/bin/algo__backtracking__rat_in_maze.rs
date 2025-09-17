#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::backtracking::rat_in_maze as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("maze_with_solution_5x5", source::tests::maze_with_solution_5x5 as fn()),
        ("maze_with_solution_6x6", source::tests::maze_with_solution_6x6 as fn()),
        ("maze_with_solution_8x8", source::tests::maze_with_solution_8x8 as fn()),
        ("maze_without_solution_4x4", source::tests::maze_without_solution_4x4 as fn()),
        ("maze_with_solution_3x4", source::tests::maze_with_solution_3x4 as fn()),
        ("maze_without_solution_3x4", source::tests::maze_without_solution_3x4 as fn()),
        ("improper_maze_representation", source::tests::improper_maze_representation as fn()),
        ("out_of_bound_start", source::tests::out_of_bound_start as fn()),
        ("empty_maze", source::tests::empty_maze as fn()),
        ("maze_with_single_cell", source::tests::maze_with_single_cell as fn()),
        ("maze_with_one_row_and_multiple_columns", source::tests::maze_with_one_row_and_multiple_columns as fn()),
        ("maze_with_multiple_rows_and_one_column", source::tests::maze_with_multiple_rows_and_one_column as fn()),
        ("maze_with_walls_surrounding_border", source::tests::maze_with_walls_surrounding_border as fn()),
        ("maze_with_no_walls", source::tests::maze_with_no_walls as fn()),
        ("maze_with_going_back", source::tests::maze_with_going_back as fn()),
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
