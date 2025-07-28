mod error;

pub use error::SubCommandError;

use std::result::Result;

pub trait SubCommand {
    fn get_name() -> &'static str;
    fn build() -> clap::Command;
    async fn run(
        runtime: &dsot_runtime::Runtime,
        global_args: &clap::ArgMatches,
        cmd_args: &clap::ArgMatches,
    ) -> Result<(), SubCommandError>;
}

macro_rules! declare_subcommands {
    ($($name:ident),*$(,)?) => {
        $(
            mod $name;
        )*

        paste::paste! {
            pub fn register(cmd: clap::Command) -> clap::Command {
                cmd
                $(.subcommand($name::[<$name:camel Command>]::build()))*
            }
        }

        paste::paste! {
            pub async fn execute(
                runtime: &dsot_runtime::Runtime,
                global_args: &clap::ArgMatches,
            ) -> Result<(), SubCommandError> {
                $(
                    if let Some(config_args) = global_args.subcommand_matches($name::[<$name:camel Command>]::get_name()) {
                        return $name::[<$name:camel Command>]::run(runtime, global_args, config_args).await;
                    }
                )*

                Err(SubCommandError::InvalidCommand())
            }
        }
    }
}

declare_subcommands![config, user,];
