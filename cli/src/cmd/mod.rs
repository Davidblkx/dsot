#[macro_use]
pub(crate) mod infra;

pub(crate) mod error;

mod commands;

pub mod globals;
pub use globals::*;

use clap::Command;

pub fn create_app() -> Command {
    let cmd = Command::new("dsot")
        .version(env!("CARGO_PKG_VERSION"))
        .author("David Pires <dev@davidpires.pt>")
        .about("DSOT - Music organization management tool");

    commands::register_commands(globals::register(cmd))
        .subcommand(commands::server::build_server_command())
}

pub async fn execute(runtime: dsot_runtime::Runtime, args: clap::ArgMatches) -> usize {
    let (runtime, cmd_result) = if let Some(server_context) =
        commands::server::get_server_context(&args)
    {
        let server_result = commands::server::execute_server_command(runtime, server_context).await;
        (None, server_result)
    } else {
        let cmd_result = commands::execute(&runtime, &args).await;
        (Some(runtime), cmd_result)
    };

    let exit_code = match cmd_result {
        Ok(_) => {
            log::trace!("Command executed successfully.");
            0
        }
        Err(e) => {
            if let Some(message) = e.message {
                log::error!("Command execution failed: {}", message);
            } else {
                log::error!("Command execution failed with unknown error.");
            }

            e.code.unwrap_or(1)
        }
    };

    if let Some(rt) = runtime {
        rt.shutdown(exit_code);
    }

    exit_code
}
