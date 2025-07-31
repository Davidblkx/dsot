generate_subcommands!(add, list,);

pub struct InboxCommand;

impl AppCommand for InboxCommand {
    fn name() -> &'static str {
        "inbox"
    }

    fn build() -> clap::Command {
        let cmd = clap::Command::new(InboxCommand::name())
            .about("Inbox helps keeping track of tracks, albuns or artits you want to explore");

        register_commands(cmd)
    }

    async fn execute(runtime: &dsot_runtime::Runtime, context: AppCommandContext) -> AppResult<()> {
        execute(runtime, context).await
    }
}
