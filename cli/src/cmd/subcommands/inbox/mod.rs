mod add;

use super::{SubCommand, SubCommandError};

pub struct InboxCommand;

impl SubCommand for InboxCommand {
    fn get_name() -> &'static str {
        "inbox"
    }

    fn build() -> clap::Command {
        todo!()
    }

    async fn run(
        _runtime: &dsot_runtime::Runtime,
        _global_args: &clap::ArgMatches,
        _cmd_args: &clap::ArgMatches,
    ) -> Result<(), SubCommandError> {
        todo!()
    }
}
