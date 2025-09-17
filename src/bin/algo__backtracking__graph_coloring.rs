#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::backtracking::graph_coloring as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_complete_graph_with_3_colors", source::tests::test_complete_graph_with_3_colors as fn()),
        ("test_linear_graph_with_2_colors", source::tests::test_linear_graph_with_2_colors as fn()),
        ("test_incomplete_graph_with_insufficient_colors", source::tests::test_incomplete_graph_with_insufficient_colors as fn()),
        ("test_empty_graph", source::tests::test_empty_graph as fn()),
        ("test_non_square_matrix", source::tests::test_non_square_matrix as fn()),
        ("test_single_vertex_graph", source::tests::test_single_vertex_graph as fn()),
        ("test_bipartite_graph_with_2_colors", source::tests::test_bipartite_graph_with_2_colors as fn()),
        ("test_large_graph_with_3_colors", source::tests::test_large_graph_with_3_colors as fn()),
        ("test_disconnected_graph", source::tests::test_disconnected_graph as fn()),
        ("test_no_valid_coloring", source::tests::test_no_valid_coloring as fn()),
        ("test_more_colors_than_nodes", source::tests::test_more_colors_than_nodes as fn()),
        ("test_no_coloring_with_zero_colors", source::tests::test_no_coloring_with_zero_colors as fn()),
        ("test_complete_graph_with_3_vertices_and_3_colors", source::tests::test_complete_graph_with_3_vertices_and_3_colors as fn()),
        ("test_directed_graph_with_3_colors", source::tests::test_directed_graph_with_3_colors as fn()),
        ("test_directed_graph_no_valid_coloring", source::tests::test_directed_graph_no_valid_coloring as fn()),
        ("test_large_directed_graph_with_3_colors", source::tests::test_large_directed_graph_with_3_colors as fn()),
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
