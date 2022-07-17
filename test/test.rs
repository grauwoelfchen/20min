mod args;
mod opts;

use std::env;
use std::process::{Command, Output};

const DEFAULT_CARGO_TARGET_DIR: &str = "./target";

fn run(args: &[&str]) -> Result<Output, std::io::Error> {
    let target_dir = env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| DEFAULT_CARGO_TARGET_DIR.to_string());
    let program = format!("{}/debug/20min", target_dir);

    Command::new(&program).args(args).output()
}
