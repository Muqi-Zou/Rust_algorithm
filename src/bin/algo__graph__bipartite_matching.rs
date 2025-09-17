#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::graph::bipartite_matching as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("small_graph_kuhn", source::tests::small_graph_kuhn as fn()),
        ("small_graph_hopcroft", source::tests::small_graph_hopcroft as fn()),
        ("super_small_graph_kuhn", source::tests::super_small_graph_kuhn as fn()),
        ("super_small_graph_hopcroft", source::tests::super_small_graph_hopcroft as fn()),
        ("only_one_vertex_graph_kuhn", source::tests::only_one_vertex_graph_kuhn as fn()),
        ("only_one_vertex_graph_hopcroft", source::tests::only_one_vertex_graph_hopcroft as fn()),
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
