#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::graph::prim as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("empty", source::tests::empty as fn()),
        ("single_vertex", source::tests::single_vertex as fn()),
        ("single_edge", source::tests::single_edge as fn()),
        ("tree_1", source::tests::tree_1 as fn()),
        ("tree_2", source::tests::tree_2 as fn()),
        ("tree_3", source::tests::tree_3 as fn()),
        ("graph_1", source::tests::graph_1 as fn()),
        ("graph_2", source::tests::graph_2 as fn()),
        ("graph_3", source::tests::graph_3 as fn()),
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
