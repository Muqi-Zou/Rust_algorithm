#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::general::permutations::steinhaus_johnson_trotter as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
