#[macro_use]
mod macros;
mod subcommands;

pub mod globals;
pub use globals::*;

use clap::Command;

pub fn create_app() -> Command {
    let cmd = Command::new("dsot")
        .version(env!("CARGO_PKG_VERSION"))
        .author("David Pires <dev@davidpires.pt>")
        .about("DSOT - Music organization management tool");

    subcommands::register(globals::register(cmd))
}

pub async fn execute(
    runtime: &dsot_runtime::Runtime,
    args: clap::ArgMatches,
) -> Result<(), subcommands::SubCommandError> {
    subcommands::execute(runtime, &args).await
}
