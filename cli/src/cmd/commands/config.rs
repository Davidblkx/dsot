use bakunin_config::{Value, config_layer::ConfigLayer};
use dsot_runtime::infra::config::{
    CUSTOM_FILE_LAYER_NAME, GLOBAL_FILE_LAYER_NAME, LOCAL_FILE_LAYER_NAME,
};

use crate::cmd::error::{AppError, AppResult};
use crate::cmd::infra::{AppCommand, CommandArgs};
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

impl AppCommand for ConfigCommand {
    fn name() -> &'static str {
        NAME
    }

    fn build() -> clap::Command {
        clap::Command::new(Self::name())
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

    async fn execute(runtime: &dsot_runtime::Runtime, args: CommandArgs) -> AppResult<()> {
        if let (Some(key), Some(value)) = (
            args.command.get_one::<String>("key"),
            args.command.get_one::<String>("value"),
        ) {
            let create = InitArg::enabled(&args.command);
            if UseGlobalArg::enabled(&args.command) {
                if let Some(layer) = runtime.config.handler.get_layer(GLOBAL_FILE_LAYER_NAME) {
                    log::trace!("Writing [{}] to global config", key);
                    return write_to_layer(layer, key, value.clone(), create);
                } else {
                    return AppError::ConfigError()
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
                return AppError::ConfigError()
                    .with_message("Local config file not defined")
                    .to_err();
            }
        } else if let Some(key) = args.command.get_one::<String>("key") {
            let value = runtime.config.get_config_value(key);
            print_message(&args.global, format!("{:?}", value));
        } else {
            return AppError::MissingArgument()
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
) -> AppResult<()> {
    if layer.has_value() {
        let mut v: Value = layer.read_value()?;

        v.set(key, Value::String(value))
            .map_err(|e| AppError::ConfigError().with_message(e.to_string()))?;
        layer.write_value(&v)?;
    } else if create {
        let mut v = Value::new_map();
        v.set(key, Value::String(value))
            .map_err(|e| AppError::ConfigError().with_message(e.to_string()))?;
        layer.write_value(&v)?;
    } else {
        return AppError::ConfigError()
            .with_message("Config file doesn't exist, use -i to create it")
            .to_err();
    }

    Ok(())
}
