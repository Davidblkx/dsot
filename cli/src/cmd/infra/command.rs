use crate::cmd::error::AppResult;
use clap::ArgMatches;
use dsot_runtime::Runtime;

pub struct AppCommandContext {
    pub global: clap::ArgMatches,
    pub args: clap::ArgMatches,
}

pub trait AppCommand {
    fn name() -> &'static str;
    fn build() -> clap::Command;
    async fn execute(runtime: &Runtime, context: AppCommandContext) -> AppResult<()>;
}

pub trait MatchCommand {
    fn match_command(&self, name: &'static str) -> Option<AppCommandContext>;
}

impl MatchCommand for AppCommandContext {
    fn match_command(&self, name: &'static str) -> Option<AppCommandContext> {
        let args = self.args.subcommand_matches(name).map(|e| e.clone());

        match args {
            Some(command) => Some(AppCommandContext {
                global: self.global.clone(),
                args: command,
            }),
            None => None,
        }
    }
}

impl MatchCommand for ArgMatches {
    fn match_command(&self, name: &'static str) -> Option<AppCommandContext> {
        let args = self.subcommand_matches(name).map(|c| c.clone());

        match args {
            Some(command) => Some(AppCommandContext {
                global: self.clone(),
                args: command,
            }),
            None => None,
        }
    }
}
