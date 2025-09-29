use dsot_core::model::entities::user::User;
use dsot_runtime::{Runtime, Users};
use tauri::{State, async_runtime::Mutex};

use crate::error::CommandResult;

#[tauri::command]
pub async fn users_list(state: State<'_, Mutex<Runtime>>) -> Result<CommandResult<Vec<User>>, ()> {
    let runtime = state.lock().await;

    let users = runtime.list_users().await;

    Ok(CommandResult::handle_runtime(users))
}
