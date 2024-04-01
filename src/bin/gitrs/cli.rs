use clap::Command;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn cli() -> Command {
    Command::new(NAME)
        .version(VERSION)
}

pub mod error {
    use std::fmt;
    use std::process::ExitCode;

    pub type CliResult = Result<(), CliError>;

    #[derive(Debug)]
    pub enum CliError {
        NoCommand,
        Other { error: anyhow::Error },
    }
}
