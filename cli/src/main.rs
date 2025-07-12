mod cmd;

use std::path::PathBuf;

use dsot_runtime::{
    Config,
    infra::{
        config::{ConfigOptions, LogConfig},
        init_runtime_logger,
    },
    init,
};

#[tokio::main]
async fn main() {
    // User agent used for MusicBrainz API requests
    music_brainz::init_user_agent("dsot", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    // Parse command line arguments
    let args = cmd::create_app().get_matches();

    // If debug or debug_folder flags are set, initialize the logger
    // otherwise, it will be handled by the runtime logger
    let log_handler = if args.get_flag(cmd::ARG_DEBUG) || args.get_flag(cmd::ARG_DEBUG_FOLDER) {
        init_runtime_logger(&LogConfig {
            enabled: true,
            use_console: true,
            use_file: args.get_flag(cmd::ARG_DEBUG_FOLDER),
            to_stderr: true,
            to_folder: "./dsot_logs".into(),
            level: "trace".to_string(),
            file_level: None,
            console_level: None,
        })
    } else {
        None
    };

    let mut config_options = ConfigOptions::default();

    // If the --config argument is provided, load the specified configuration file
    // and disable search for default config files
    if let Some(config_file) = args.get_one::<PathBuf>(cmd::ARG_CONFIG) {
        config_options.config_path = Some(config_file.to_str().unwrap().to_string());
        config_options.search = false;
    }
    let config = Config::create(config_options).expect("Failed to create configuration");

    // Initialize the runtime with the loaded configuration
    let runtime = match init(config).await {
        Ok(runtime) => {
            log::debug!("Runtime initialized successfully.");
            runtime
        }
        Err(e) => {
            log::error!("Failed to initialize runtime: {}", e);
            panic!("Runtime initialization failed");
        }
    };

    // Execute the command with the provided arguments
    match cmd::execute(&runtime, args).await {
        Ok(_) => log::trace!("Command executed successfully."),
        Err(e) => log::error!("Error executing command: {}", e),
    }

    // Shutdown the runtime and logger if they were initialized
    runtime.shutdown();
    if let Some(handler) = log_handler {
        handler.shutdown();
    }
}
