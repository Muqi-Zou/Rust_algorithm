#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::jaro_winkler_distance as source;
pub use source::*;

fn main() {
    let _ = core::hint::black_box(());
}
