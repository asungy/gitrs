use crate::cli::error::CliResult;
use clap::{
    Arg,
    ArgAction,
    ArgMatches,
    Command,
};

pub(super) const NAME: &str = "cat-file";

pub fn cli() -> Command {
    clap::Command::new(NAME)
        .about("Provide contents or details of repository objects")
        .arg(Arg::new("pretty-print")
            .short('p')
            .action(ArgAction::SetTrue)
            .help("pretty-print <object> content")
        )
        .arg(Arg::new("object")
             .index(1)
             .action(ArgAction::Set)
             .help("The name of the object to show.")
             .required(true)
         )
}

pub fn exec(args: &ArgMatches) -> CliResult {
    let ppflag: bool = *args.get_one::<bool>("pretty-print").unwrap_or(&false);
    let object = args.get_one::<String>("object").unwrap();

    if ppflag {
        crate::core::cat_file::exec(object)?;
    } else {
        cli().print_help().map_err(|e| anyhow::Error::from(e))?;
    }

    Ok(())
}
