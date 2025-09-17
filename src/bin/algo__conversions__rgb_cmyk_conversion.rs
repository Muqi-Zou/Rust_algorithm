#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::conversions::rgb_cmyk_conversion as source;

fn main() {
    println!("conversions/rgb_cmyk_conversion.rs");
}
