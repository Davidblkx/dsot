use crate::cmd::error::AppResult;
use crate::cmd::infra::{AppCommand, AppCommandContext};

pub struct AddCommand;

impl AppCommand for AddCommand {
    fn name() -> &'static str {
        "add"
    }

    fn build() -> clap::Command {
        clap::Command::new(AddCommand::name())
    }

    async fn execute(
        _runtime: &dsot_runtime::Runtime,
        _context: AppCommandContext,
    ) -> AppResult<()> {
        todo!()
    }
}
