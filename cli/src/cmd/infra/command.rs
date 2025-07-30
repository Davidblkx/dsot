use crate::cmd::error::AppResult;
use clap::ArgMatches;
use dsot_runtime::Runtime;

pub struct CommandArgs {
    pub global: clap::ArgMatches,
    #[allow(unused)]
    pub parent: Option<clap::ArgMatches>,
    pub command: clap::ArgMatches,
}

pub trait AppCommand {
    fn name() -> &'static str;
    fn build() -> clap::Command;
    async fn execute(runtime: &Runtime, args: CommandArgs) -> AppResult<()>;
}

pub trait MatchCommand {
    fn match_command(&self, name: &'static str) -> Option<CommandArgs>;
}

impl MatchCommand for CommandArgs {
    fn match_command(&self, name: &'static str) -> Option<CommandArgs> {
        let args = self.command.subcommand_matches(name).map(|e| e.clone());

        match args {
            Some(command) => Some(CommandArgs {
                global: self.global.clone(),
                command,
                parent: Some(self.command.clone()),
            }),
            None => None,
        }
    }
}

impl MatchCommand for ArgMatches {
    fn match_command(&self, name: &'static str) -> Option<CommandArgs> {
        let args = self.subcommand_matches(name).map(|c| c.clone());

        match args {
            Some(command) => Some(CommandArgs {
                global: self.clone(),
                command,
                parent: None,
            }),
            None => None,
        }
    }
}
