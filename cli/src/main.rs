mod cmd;

mod print;

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
    let log_handler = if cmd::DebugArg::enabled(&args) || cmd::DebugFolderArg::enabled(&args) {
        init_runtime_logger(&LogConfig {
            enabled: true,
            use_console: true,
            use_file: cmd::DebugFolderArg::enabled(&args),
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
    // and disable search for default config files, so it only uses the provided file
    if let Some(config_file) = cmd::ConfigArg::get(&args) {
        config_options.config_path = Some(config_file.to_str().unwrap().to_string());
        config_options.search = false;
    }
    // If the --layer-config argument is provided, set the custom layer configuration file
    // and enable search for default config files, so it can still find the default config
    else if let Some(layer_config_file) = cmd::LayerConfigArg::get(&args) {
        config_options.config_path = Some(layer_config_file.to_str().unwrap().to_string());
        config_options.search = true;
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
    let exit_code = match cmd::execute(&runtime, args).await {
        Ok(_) => {
            log::trace!("Command executed successfully.");
            0
        }
        Err(e) => {
            if let Some(message) = e.message {
                log::error!("Command execution failed: {}", message);
            } else {
                log::error!("Command execution failed with unknown error.");
            }

            e.code.unwrap_or(1)
        }
    };

    // Shutdown the runtime and related resources
    runtime.shutdown(exit_code);

    // Clean up the local logger if it was initialized
    if let Some(handler) = log_handler {
        handler.shutdown();
    }

    std::process::exit(exit_code.try_into().unwrap_or(-1));
}
