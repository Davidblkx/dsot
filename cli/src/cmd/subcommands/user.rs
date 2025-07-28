use super::{SubCommand, SubCommandError};
use crate::print::print_message;
use dsot_runtime::Users;

declare_arg_bool!(ListUsersArg, "list", short: 'l', "List existing users");
declare_arg_string!(CreateUserArg, "create", "Create a new user");

static NAME: &str = "user";

pub struct UserCommand;

impl SubCommand for UserCommand {
    fn get_name() -> &'static str {
        NAME
    }

    fn build() -> clap::Command {
        clap::Command::new(Self::get_name())
            .about("Handle users creation and list existing users")
            .arg(ListUsersArg::build())
            .arg(CreateUserArg::build())
    }

    async fn run(
        runtime: &dsot_runtime::Runtime,
        global_args: &clap::ArgMatches,
        cmd_args: &clap::ArgMatches,
    ) -> Result<(), SubCommandError> {
        if let Some(user_name) = CreateUserArg::get(cmd_args) {
            let id = runtime.create_user(user_name).await?;
            print_message(global_args, id.to_string());
        }

        if ListUsersArg::enabled(cmd_args) {
            let users = runtime.list_users().await?;
            for u in users {
                print_message(global_args, format!("{} - {}", u.id, u.name));
            }
        }

        Ok(())
    }
}
