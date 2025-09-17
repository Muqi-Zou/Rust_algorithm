#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::math::linear_sieve as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
