#![allow(clippy::all, missing_debug_implementations)]

mod cli;
pub mod core;

use std::process::ExitCode;

fn main() -> ExitCode {
    match cli::exec() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => error.exit_code(),
    }
}
