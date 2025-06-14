mod config;

use std::path::PathBuf;

use clap::{arg, value_parser, Command, Arg};

use config::ConfigCmd;

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

    return cmd
        .subcommand(ConfigCmd::build());
}

pub async fn execute(runtime: &dsot_runtime::Runtime, args: clap::ArgMatches) -> anyhow::Result<()> {
    if let Some(config_args) = args.subcommand_matches(ConfigCmd::get_name()) {
        return ConfigCmd::run(runtime, config_args);
    }

    Err(anyhow::anyhow!("No valid command provided. Use --help for more information."))
}
