#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::data_structures::probabilistic::count_min_sketch as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
