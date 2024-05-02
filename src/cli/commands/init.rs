use crate::cli::error::CliResult;
use clap::{ArgMatches, Command};

pub(super) const NAME: &str = "init";

pub fn cli() -> Command {
    clap::Command::new(NAME).about("Create an empty git repository.")
}

pub fn exec(_args: &ArgMatches) -> CliResult {
    crate::core::init::exec().map_err(|e| e.into())
}
