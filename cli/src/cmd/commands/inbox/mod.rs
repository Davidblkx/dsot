generate_subcommands!(add);

pub struct InboxCommand;

impl AppCommand for InboxCommand {
    fn name() -> &'static str {
        "inbox"
    }

    fn build() -> clap::Command {
        let cmd = clap::Command::new(InboxCommand::name());

        register_commands(cmd)
    }

    async fn execute(runtime: &dsot_runtime::Runtime, args: CommandArgs) -> AppResult<()> {
        execute(runtime, args).await
    }
}
