#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::sorting::binary_insertion_sort as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
