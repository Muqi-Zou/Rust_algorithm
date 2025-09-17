#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::graph::eulerian_path as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_eulerian_cycle", source::tests::test_eulerian_cycle as fn()),
        ("test_simple_path", source::tests::test_simple_path as fn()),
        ("test_disconnected_graph", source::tests::test_disconnected_graph as fn()),
        ("None", source::tests::None as fn()),
        ("test_single_cycle", source::tests::test_single_cycle as fn()),
        ("test_empty_graph", source::tests::test_empty_graph as fn()),
        ("None", source::tests::None as fn()),
        ("test_unbalanced_path", source::tests::test_unbalanced_path as fn()),
        ("test_no_eulerian_path", source::tests::test_no_eulerian_path as fn()),
        ("None", source::tests::None as fn()),
        ("test_complex_eulerian_path", source::tests::test_complex_eulerian_path as fn()),
        ("test_single_node_self_loop", source::tests::test_single_node_self_loop as fn()),
        ("test_complete_graph", source::tests::test_complete_graph as fn()),
        ("test_multiple_disconnected_components", source::tests::test_multiple_disconnected_components as fn()),
        ("None", source::tests::None as fn()),
        ("test_unbalanced_graph_with_path", source::tests::test_unbalanced_graph_with_path as fn()),
        ("test_node_with_no_edges", source::tests::test_node_with_no_edges as fn()),
        ("test_multiple_edges_between_same_nodes", source::tests::test_multiple_edges_between_same_nodes as fn()),
        ("test_larger_graph_with_eulerian_path", source::tests::test_larger_graph_with_eulerian_path as fn()),
        ("test_no_edges_multiple_nodes", source::tests::test_no_edges_multiple_nodes as fn()),
        ("None", source::tests::None as fn()),
        ("test_multiple_start_and_end_nodes", source::tests::test_multiple_start_and_end_nodes as fn()),
        ("None", source::tests::None as fn()),
        ("test_single_edge", source::tests::test_single_edge as fn()),
        ("test_multiple_eulerian_paths", source::tests::test_multiple_eulerian_paths as fn()),
        ("test_dag_path", source::tests::test_dag_path as fn()),
        ("test_parallel_edges_case", source::tests::test_parallel_edges_case as fn()),
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
