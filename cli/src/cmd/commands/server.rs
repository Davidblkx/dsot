use std::sync::Arc;

use crate::cmd::{
    error::{AppError, AppResult},
    infra::{AppCommandContext, MatchCommand},
};

declare_arg_number!(ServerPortArg(u16), "port", "Port to run the server on", 'p');

pub static NAME: &str = "server";

pub fn build_server_command() -> clap::Command {
    clap::Command::new(NAME)
        .about("Run the DSOT server")
        .arg(ServerPortArg::build())
}

pub fn get_server_context(args: &clap::ArgMatches) -> Option<AppCommandContext> {
    args.match_command(NAME)
}

pub async fn execute_server_command(
    runtime: dsot_runtime::Runtime,
    context: AppCommandContext,
) -> AppResult<()> {
    let port = ServerPortArg::get(&context.args).unwrap_or(&6677);

    match dsot_server::run_server(dsot_server::ServerOptions {
        runtime: Arc::new(runtime),
        port: *port,
    })
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(AppError::from(err)),
    }
}
