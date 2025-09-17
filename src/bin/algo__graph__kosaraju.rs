#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::graph::kosaraju as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("test_kosaraju_single_sccs", source::tests::test_kosaraju_single_sccs as fn()),
        ("test_kosaraju_multiple_sccs", source::tests::test_kosaraju_multiple_sccs as fn()),
        ("test_kosaraju_multiple_sccs1", source::tests::test_kosaraju_multiple_sccs1 as fn()),
        ("test_kosaraju_no_scc", source::tests::test_kosaraju_no_scc as fn()),
        ("test_kosaraju_empty_graph", source::tests::test_kosaraju_empty_graph as fn()),
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
