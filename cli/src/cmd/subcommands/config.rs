use bakunin_config::{Value, config_layer::ConfigLayer};
use dsot_runtime::infra::config::{
    CUSTOM_FILE_LAYER_NAME, GLOBAL_FILE_LAYER_NAME, LOCAL_FILE_LAYER_NAME,
};

use super::{SubCommand, SubCommandError};
use crate::print::print_message;

static NAME: &str = "config";

declare_arg_bool!(UseGlobalArg, "global", "Write to global configuration", 'g');
declare_arg_bool!(
    InitArg,
    "init",
    "Creates configuration file if not present",
    'i'
);

pub struct ConfigCommand;

impl SubCommand for ConfigCommand {
    fn get_name() -> &'static str {
        NAME
    }

    fn build() -> clap::Command {
        clap::Command::new(Self::get_name())
            .about("Read configuration values")
            .arg(UseGlobalArg::build())
            .arg(InitArg::build())
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
        global_args: &clap::ArgMatches,
        cmd_args: &clap::ArgMatches,
    ) -> Result<(), SubCommandError> {
        if let (Some(key), Some(value)) = (
            cmd_args.get_one::<String>("key"),
            cmd_args.get_one::<String>("value"),
        ) {
            let create = InitArg::enabled(cmd_args);
            if UseGlobalArg::enabled(cmd_args) {
                if let Some(layer) = runtime.config.handler.get_layer(GLOBAL_FILE_LAYER_NAME) {
                    log::trace!("Writing [{}] to global config", key);
                    return write_to_layer(layer, key, value.clone(), create);
                } else {
                    return SubCommandError::ConfigError()
                        .with_message("Global config file not defined")
                        .to_err();
                }
            } else if let Some(layer) = runtime.config.handler.get_layer(CUSTOM_FILE_LAYER_NAME) {
                log::trace!("Writing [{}] to config file", key);
                return write_to_layer(layer, key, value.clone(), create);
            } else if let Some(layer) = runtime.config.handler.get_layer(LOCAL_FILE_LAYER_NAME) {
                log::trace!("Writing [{}] to config file", key);
                return write_to_layer(layer, key, value.clone(), create);
            } else {
                return SubCommandError::ConfigError()
                    .with_message("Local config file not defined")
                    .to_err();
            }
        } else if let Some(key) = cmd_args.get_one::<String>("key") {
            let value = runtime.config.get_config_value(key);
            print_message(&global_args, format!("{:?}", value));
        } else {
            return SubCommandError::MissingArgument()
                .with_message("The 'key' argument is required.")
                .to_err();
        }

        Ok(())
    }
}

fn write_to_layer(
    layer: &Box<dyn ConfigLayer>,
    key: &str,
    value: String,
    create: bool,
) -> Result<(), SubCommandError> {
    if layer.has_value() {
        let mut v: Value = layer
            .read_value()
            .map_err(|e| SubCommandError::from_bakunin_error(e))?;

        v.set(key, Value::String(value))
            .map_err(|e| SubCommandError::ConfigError().with_message(e.to_string()))?;
        layer
            .write_value(&v)
            .map_err(|e| SubCommandError::from_bakunin_error(e))?;
    } else if create {
        let mut v = Value::new_map();
        v.set(key, Value::String(value))
            .map_err(|e| SubCommandError::ConfigError().with_message(e.to_string()))?;
        layer
            .write_value(&v)
            .map_err(|e| SubCommandError::from_bakunin_error(e))?;
    } else {
        return SubCommandError::ConfigError()
            .with_message("Config file doesn't exist, use -i to create it")
            .to_err();
    }

    Ok(())
}
