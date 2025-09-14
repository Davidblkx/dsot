use dsot_runtime::{Config, Runtime, error::Result, init};
use tauri::{
    Manager,
    async_runtime::{Mutex, block_on},
};

mod commands;
pub mod error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());

    commands::register_commands(builder)
        .setup(|app| {
            let runtime = block_on(async { setup_dsot().await });

            match runtime {
                Ok(rt) => {
                    app.manage(Mutex::new(rt));
                    Ok(())
                }
                Err(e) => {
                    log::error!("Failed to initialize DSOT runtime: {}", e);
                    Err(Box::new(e))
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub async fn setup_dsot() -> Result<Runtime> {
    let config = Config::default();
    init(config).await
}
