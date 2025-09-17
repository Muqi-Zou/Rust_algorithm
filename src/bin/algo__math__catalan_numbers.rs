#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::catalan_numbers as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
