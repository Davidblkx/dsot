use super::{SubCommand, SubCommandError};

static NAME: &str = "config";

pub struct ConfigCommand;

impl SubCommand for ConfigCommand {
    fn get_name() -> &'static str {
        NAME
    }

    fn build() -> clap::Command {
        clap::Command::new(Self::get_name())
            .about("Read configuration values")
            .arg(clap::arg!(--global "Write to global configuration"))
            .arg(
                clap::Arg::new("key")
                    .help("The configuration key to read")
                    .required(true),
            )
            .arg(
                clap::Arg::new("value")
                    .help("Set a new value for the configuration key")
                    .required(false),
            )
    }

    async fn run(
        runtime: &dsot_runtime::Runtime,
        _global_args: &clap::ArgMatches,
        cmd_args: &clap::ArgMatches,
    ) -> Result<(), SubCommandError> {
        if let (Some(key), Some(value)) = (
            cmd_args.get_one::<String>("key"),
            cmd_args.get_one::<String>("value"),
        ) {
            log::warn!("Setting configuration values is not supported in this command.");
            log::trace!("Key: {}, Value: {}", key, value);
        } else if let Some(key) = cmd_args.get_one::<String>("key") {
            let value = runtime.config.get_config_value(key);

            match value {
                bakunin_config::Value::None => println!("No value found for '{}'", key),
                _ => println!("{:?}", value),
            }
        } else {
            return SubCommandError::MissingArgument()
                .with_message("The 'key' argument is required.")
                .to_err();
        }

        Ok(())
    }
}
