use dsot_core::model::entities::user::User;
use dsot_runtime::{Runtime, Users};
use tauri::{Builder, Wry};
use tauri::{State, async_runtime::Mutex};

use crate::error::CommandResult;

pub fn register_user(builder: Builder<Wry>) -> Builder<Wry> {
    builder.invoke_handler(tauri::generate_handler![users_list])
}

#[tauri::command]
async fn users_list(state: State<'_, Mutex<Runtime>>) -> Result<CommandResult<Vec<User>>, ()> {
    let runtime = state.lock().await;

    let users = runtime.list_users().await;

    Ok(CommandResult::handle_runtime(users))
}
