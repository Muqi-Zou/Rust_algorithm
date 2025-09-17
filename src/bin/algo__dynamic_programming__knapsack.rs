#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::dynamic_programming::knapsack as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
