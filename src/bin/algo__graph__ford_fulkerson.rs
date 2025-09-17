#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::graph::ford_fulkerson as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_empty_graph", source::tests::test_empty_graph as fn()),
        ("test_source_out_of_bound", source::tests::test_source_out_of_bound as fn()),
        ("test_sink_out_of_bound", source::tests::test_sink_out_of_bound as fn()),
        ("test_improper_graph", source::tests::test_improper_graph as fn()),
        ("test_graph_with_small_flow", source::tests::test_graph_with_small_flow as fn()),
        ("test_graph_with_medium_flow", source::tests::test_graph_with_medium_flow as fn()),
        ("test_graph_with_large_flow", source::tests::test_graph_with_large_flow as fn()),
        ("test_complex_graph", source::tests::test_complex_graph as fn()),
        ("test_disconnected_graph", source::tests::test_disconnected_graph as fn()),
        ("test_unconnected_sink", source::tests::test_unconnected_sink as fn()),
        ("test_no_edges", source::tests::test_no_edges as fn()),
        ("test_single_vertex", source::tests::test_single_vertex as fn()),
        ("test_self_loop", source::tests::test_self_loop as fn()),
        ("test_same_source_sink", source::tests::test_same_source_sink as fn()),
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
