mod inbox;
mod user;

pub fn register_commands(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![
        inbox::inbox_list,
        inbox::inbox_create,
        inbox::inbox_update,
        inbox::inbox_delete,
        user::users_list,
    ])
}
