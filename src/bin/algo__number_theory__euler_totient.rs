#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::number_theory::euler_totient as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
