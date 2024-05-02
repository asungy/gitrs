use crate::cli::error::CliResult;
use clap::{ArgMatches, Command};

mod cat_file;
mod init;

pub fn builtin() -> Vec<Command> {
    vec![cat_file::cli(), init::cli()]
}

pub type Exec = fn(&ArgMatches) -> CliResult;

pub fn builtin_exec(cmd: &str) -> Option<Exec> {
    let f = match cmd {
        cat_file::NAME => cat_file::exec,
        init::NAME => init::exec,
        _ => return None,
    };

    Some(f)
}
