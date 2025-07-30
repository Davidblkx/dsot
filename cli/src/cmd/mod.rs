#[macro_use]
pub(crate) mod infra;

pub(crate) mod error;

mod commands;

pub mod globals;
pub use globals::*;

use clap::Command;
use error::AppResult;

pub fn create_app() -> Command {
    let cmd = Command::new("dsot")
        .version(env!("CARGO_PKG_VERSION"))
        .author("David Pires <dev@davidpires.pt>")
        .about("DSOT - Music organization management tool");

    commands::register_commands(globals::register(cmd))
}

pub async fn execute(runtime: &dsot_runtime::Runtime, args: clap::ArgMatches) -> AppResult<()> {
    commands::execute(runtime, &args).await
}
