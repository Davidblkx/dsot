macro_rules! __internal_generate_command_registration {
    ($($name:ident),*$(,)?) => {
        $(
            mod $name;
        )*

        paste::paste! {
            pub fn register_commands(cmd: clap::Command) -> clap::Command {
                cmd
                $(.subcommand($name::[<$name:camel Command>]::build()))*
            }
        }
    };
}

macro_rules! __internal_generate_command_exec {
    (COMMAND, $($name:ident),*$(,)?) => {
        paste::paste! {
            pub async fn execute(
                runtime: &dsot_runtime::Runtime,
                args: &clap::ArgMatches,
            ) -> AppResult<()> {
                $(
                    if let Some(args) = args.match_command($name::[<$name:camel Command>]::name()) {
                        return $name::[<$name:camel Command>]::execute(runtime, args).await;
                    }
                )*

                Err(AppError::InvalidCommand())
            }
        }
    };
    (SUBCOMMAND, $($name:ident),*$(,)?) => {
        paste::paste! {
            pub async fn execute(
                runtime: &dsot_runtime::Runtime,
                context: AppCommandContext,
            ) -> AppResult<()> {
                $(
                    if let Some(args) = context.match_command($name::[<$name:camel Command>]::name()) {
                        return $name::[<$name:camel Command>]::execute(runtime, args).await;
                    }
                )*

                Err(AppError::InvalidCommand())
            }
        }
    };
}

macro_rules! generate_subcommands {
    ($($name:ident),*$(,)?) => {
        use $crate::cmd::infra::{AppCommandContext, AppCommand, MatchCommand};
        use $crate::cmd::error::{AppResult, AppError};

        __internal_generate_command_registration!{$($name, )*}

        __internal_generate_command_exec!{SUBCOMMAND, $($name, )*}
    };
}

macro_rules! generate_commands {
    ($($name:ident),*$(,)?) => {
        #[allow(unused_imports)]
        use $crate::cmd::infra::{AppCommandContext, AppCommand, MatchCommand};
        use $crate::cmd::error::{AppResult, AppError};

        __internal_generate_command_registration!{$($name, )*}

        __internal_generate_command_exec!{COMMAND, $($name, )*}
    };
}
