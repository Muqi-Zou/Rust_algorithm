#![allow(warnings)]
#![allow(clippy::all)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

#[allow(unused_imports)]
use crate::graph::heavy_light_decomposition as source;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut cargo_args = Vec::new();
    let mut test_args = Vec::new();
    let mut after_delimiter = false;
    for arg in std::env::args().skip(1) {
        if !after_delimiter && arg == "--" {
            after_delimiter = true;
            continue;
        }
        if after_delimiter {
            test_args.push(arg);
        } else {
            cargo_args.push(arg);
        }
    }
    let mut command = std::process::Command::new(cargo);
    command.current_dir(manifest_dir);
    command.arg("test");
    command.arg("graph::heavy_light_decomposition::tests::");
    if !cargo_args.is_empty() {
        command.args(cargo_args);
    }
    if !test_args.is_empty() {
        command.arg("--");
        command.args(test_args);
    }
    match command.status() {
        Ok(status) if status.success() => {}
        Ok(status) => std::process::exit(status.code().unwrap_or(1)),
        Err(err) => {
            eprintln!("failed to run cargo test: {err}");
            std::process::exit(1);
        }
    }
}
