#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::backtracking::hamiltonian_cycle as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_complete_graph", source::tests::test_complete_graph as fn()),
        ("test_directed_graph_with_cycle", source::tests::test_directed_graph_with_cycle as fn()),
        ("test_undirected_graph_with_cycle", source::tests::test_undirected_graph_with_cycle as fn()),
        ("test_directed_graph_no_cycle", source::tests::test_directed_graph_no_cycle as fn()),
        ("test_undirected_graph_no_cycle", source::tests::test_undirected_graph_no_cycle as fn()),
        ("test_triangle_graph", source::tests::test_triangle_graph as fn()),
        ("test_tree_graph", source::tests::test_tree_graph as fn()),
        ("test_empty_graph", source::tests::test_empty_graph as fn()),
        ("test_improper_graph", source::tests::test_improper_graph as fn()),
        ("test_start_out_of_bound", source::tests::test_start_out_of_bound as fn()),
        ("test_complex_directed_graph", source::tests::test_complex_directed_graph as fn()),
        ("single_node_self_loop", source::tests::single_node_self_loop as fn()),
        ("single_node", source::tests::single_node as fn()),
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
