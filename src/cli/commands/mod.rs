use clap::{ArgMatches, Command};
use crate::cli::error::CliResult;

mod init;

pub fn builtin() -> Vec<Command> {
    vec![
        init::cli(),
    ]
}

pub type Exec = fn(&ArgMatches) -> CliResult;

pub fn builtin_exec(cmd: &str) -> Option<Exec> {
    let f = match cmd {
        init::NAME => init::exec,
        _ => return None,
    };

    Some(f)
}
