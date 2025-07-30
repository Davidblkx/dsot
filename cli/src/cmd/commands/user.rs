use crate::cmd::error::AppResult;
use crate::cmd::infra::{AppCommand, CommandArgs};
use crate::print::print_message;
use dsot_runtime::Users;

declare_arg_bool!(ListUsersArg, "list", "List existing users", 'l');
declare_arg_string!(CreateUserArg, "create", "Create a new user");

static NAME: &str = "user";

pub struct UserCommand;

impl AppCommand for UserCommand {
    fn name() -> &'static str {
        NAME
    }

    fn build() -> clap::Command {
        clap::Command::new(Self::name())
            .about("Handle users creation and list existing users")
            .arg(ListUsersArg::build())
            .arg(CreateUserArg::build())
    }

    async fn execute(runtime: &dsot_runtime::Runtime, args: CommandArgs) -> AppResult<()> {
        if let Some(user_name) = CreateUserArg::get(&args.command) {
            let id = runtime.create_user(user_name).await?;
            print_message(&args.global, id.to_string());
        }

        if ListUsersArg::enabled(&args.command) {
            let users = runtime.list_users().await?;
            for u in users {
                print_message(&args.global, format!("{} - {}", u.id, u.name));
            }
        }

        Ok(())
    }
}
