#![allow(warnings)]
#![allow(clippy::all)]

pub use the_algorithms_rust::string::duval_algorithm as source;
pub use source::*;

fn main() {
    let mut cargo_args = vec!["test".to_string(), "--package".to_string(), "the_algorithms_rust".to_string()];
    cargo_args.push("--lib".to_string());
    cargo_args.push("string::duval_algorithm::".to_string());
    let mut passthrough = std::env::args().skip(1).collect::<Vec<_>>();
    let harness_args = if let Some(pos) = passthrough.iter().position(|arg| arg == "--") {
        let mut rest = passthrough.split_off(pos);
        rest.remove(0);
        rest
    } else {
        Vec::new()
    };
    cargo_args.extend(passthrough);
    let mut command = std::process::Command::new("cargo");
    command.args(&cargo_args);
    if !harness_args.is_empty() {
        command.arg("--");
        command.args(&harness_args);
    }
    match command.status() {
        Ok(status) if status.success() => {}
        Ok(status) => std::process::exit(status.code().unwrap_or(1)),
        Err(error) => {
            eprintln!("failed to run cargo test: {}", error);
            std::process::exit(1);
        }
    }
}
