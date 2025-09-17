#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::graph::minimum_spanning_tree as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_seven_vertices_eleven_edges", source::tests::test_seven_vertices_eleven_edges as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("test_ten_vertices_twenty_edges", source::tests::test_ten_vertices_twenty_edges as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("test_disconnected_graph", source::tests::test_disconnected_graph as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
        ("Edge", source::tests::Edge as fn()),
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
