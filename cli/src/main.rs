use clap::Command;
use dsot_runtime::{
    Config,
    infra::{config::LogConfig},
    infra::config_load::ConfigLoader,
    init,
};

#[tokio::main]
async fn main() {
    music_brainz::init_user_agent("dsot", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let config = ConfigLoader::new().load_config()
        .expect("Failed to load configuration");


    let mut config = Config::from_value(config);
    config.logger = Some(LogConfig {
        enabled: true,
        level: "trace".to_string(),
        use_file: false,
        use_console: true,
        file_level: None,
        console_level: None,
        to_folder: "./".into(),
        to_stderr: false,
    });

    let runtime = init(config).await.expect("Failed to initialize runtime");

    runtime.shutdown();
}
