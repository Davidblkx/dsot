use dsot_core::model::entities::inbox::Inbox;
use dsot_runtime::{InboxInsertValue, InboxItems, Runtime};
use tauri::{State, async_runtime::Mutex};
use uuid::Uuid;

use crate::error::CommandResult;

#[derive(serde::Deserialize)]
pub struct ListInboxParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[tauri::command]
pub async fn inbox_list(
    state: State<'_, Mutex<Runtime>>,
    input: ListInboxParams,
) -> Result<CommandResult<Vec<Inbox>>, ()> {
    let runtime = state.lock().await;
    let length = input.limit.unwrap_or(10).max(0);
    let skip = input.offset.unwrap_or(0).max(0);

    let inbox = runtime.list_inbox(length, skip).await;

    Ok(CommandResult::handle_runtime(inbox))
}

#[tauri::command]
pub async fn inbox_update(
    state: State<'_, Mutex<Runtime>>,
    input: Inbox,
) -> Result<CommandResult<bool>, ()> {
    let runtime = state.lock().await;
    let inbox = runtime.update_inbox(input).await;

    Ok(CommandResult::handle_runtime(inbox))
}

#[tauri::command]
pub async fn inbox_create(
    state: State<'_, Mutex<Runtime>>,
    input: InboxInsertValue,
) -> Result<CommandResult<Uuid>, ()> {
    let runtime = state.lock().await;
    let inbox = runtime.add_inbox(input).await;

    Ok(CommandResult::handle_runtime(inbox))
}

#[tauri::command]
pub async fn inbox_delete(
    state: State<'_, Mutex<Runtime>>,
    input: Inbox,
) -> Result<CommandResult<bool>, ()> {
    let runtime = state.lock().await;
    let inbox = runtime.delete_inbox(input.id).await;

    Ok(CommandResult::handle_runtime(inbox))
}
