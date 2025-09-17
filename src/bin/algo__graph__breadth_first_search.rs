#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::graph::breadth_first_search as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("breadth_first_search_graph1_when_node_not_found_returns_none", source::tests::breadth_first_search_graph1_when_node_not_found_returns_none as fn()),
        ("breadth_first_search_graph1_when_target_8_should_evaluate_all_nodes_first", source::tests::breadth_first_search_graph1_when_target_8_should_evaluate_all_nodes_first as fn()),
        ("breadth_first_search_graph2_when_no_path_to_node_returns_none", source::tests::breadth_first_search_graph2_when_no_path_to_node_returns_none as fn()),
        ("breadth_first_search_graph2_should_find_path_from_4_to_1", source::tests::breadth_first_search_graph2_should_find_path_from_4_to_1 as fn()),
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
