use super::SubCommandError;

pub trait SubCommand {
    fn get_name() -> &'static str;
    fn build() -> clap::Command;
    async fn run(
        runtime: &dsot_runtime::Runtime,
        global_args: &clap::ArgMatches,
        cmd_args: &clap::ArgMatches,
    ) -> Result<(), SubCommandError>;
}
