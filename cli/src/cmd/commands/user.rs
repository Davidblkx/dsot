use crate::cmd::error::AppResult;
use crate::cmd::infra::{AppCommand, AppCommandContext};
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

    async fn execute(runtime: &dsot_runtime::Runtime, context: AppCommandContext) -> AppResult<()> {
        if let Some(user_name) = CreateUserArg::get(&context.args) {
            let id = runtime.create_user(user_name).await?;
            context.print_message(id);
        }

        if ListUsersArg::enabled(&context.args) {
            let users = runtime.list_users().await?;
            for u in users {
                context.print_message(format!("{} - {}", u.id, u.name));
            }
        }

        Ok(())
    }
}
