#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::graph::floyd_warshall as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
