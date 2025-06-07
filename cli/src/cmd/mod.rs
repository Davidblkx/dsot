use std::path::PathBuf;

use clap::{arg, value_parser, Command, Arg};

pub static ARG_DEBUG: &str = "debug";
pub static ARG_DEBUG_FOLDER: &str = "debug-folder";
pub static ARG_CONFIG: &str = "config";

pub fn create_app() -> Command {
    let cmd = Command::new("dsot")
        .version(env!("CARGO_PKG_VERSION"))
        .author("David Pires <dev@davidpires.pt>")
        .about("DSOT - Music organization management tool")
        .args(&[
            arg!(-d --debug "enable debug mode"),
            Arg::new(ARG_DEBUG_FOLDER)
                .long(ARG_DEBUG_FOLDER)
                .help("Enable debug mode")
                .action(clap::ArgAction::SetTrue),
            arg!(-c --config <FILE> "set the configuration file")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        ]);

    cmd
}

pub async fn execute(_runtime: &dsot_runtime::Runtime, args: clap::ArgMatches) {
    match args.subcommand() {
        // Add subcommands here
        _ => {
            eprintln!("No valid command provided. Use --help for more information.");
        }
    }
}
