#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::compression::move_to_front as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
