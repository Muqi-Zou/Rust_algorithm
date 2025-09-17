#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::data_structures::avl_tree as source;
pub use source::*;

fn main() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    const TESTS: &[(&str, fn())] = &[
        ("len", source::tests::len as fn()),
        ("contains", source::tests::contains as fn()),
        ("insert", source::tests::insert as fn()),
        ("remove", source::tests::remove as fn()),
        ("sorted", source::tests::sorted as fn()),
        ("balanced", source::tests::balanced as fn()),
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
