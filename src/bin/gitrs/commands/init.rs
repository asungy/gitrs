use clap::{ArgMatches, Command};
use crate::cli::error::CliResult;

pub(super) const NAME: &str = "init";

pub fn cli() -> Command {
    clap::Command::new(NAME)
        .about("Create an empty git repository.")
}

pub fn exec(_args: &ArgMatches) -> CliResult {
    gitrs::init::exec().map_err(|error| error.into())
}
