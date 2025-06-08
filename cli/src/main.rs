mod cmd;

use std::path::PathBuf;

use dsot_runtime::{
    Config,
    infra::{config::LogConfig, config_load::ConfigLoader, init_runtime_logger},
    init,
};

#[tokio::main]
async fn main() {
    music_brainz::init_user_agent("dsot", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let args = cmd::create_app().get_matches();

    if args.get_flag(cmd::ARG_DEBUG) || args.get_flag(cmd::ARG_DEBUG_FOLDER) {
        init_runtime_logger(&LogConfig {
            enabled: true,
            use_console: true,
            use_file: args.get_flag(cmd::ARG_DEBUG_FOLDER),
            to_stderr: true,
            to_folder: "./dsot_logs".into(),
            level: "trace".to_string(),
            file_level: None,
            console_level: None,
        });
    }

    let mut loader = ConfigLoader::new();

    if let Some(config_file) = args.get_one::<PathBuf>(cmd::ARG_CONFIG) {
        loader.config_path = Some(config_file.to_str().unwrap().to_string());
    }

    let config = Config::from_value(loader.load_config().expect("Failed to load configuration"));
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

    cmd::execute(&runtime, args).await;

    runtime.shutdown();
}
