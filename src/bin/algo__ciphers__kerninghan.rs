#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::ciphers::kerninghan as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
