#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::searching::binary_search_recursive as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
