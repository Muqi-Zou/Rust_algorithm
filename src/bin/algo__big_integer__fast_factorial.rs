#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::big_integer::fast_factorial as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
