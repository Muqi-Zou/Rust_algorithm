#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::searching::jump_search as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
