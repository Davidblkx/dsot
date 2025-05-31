use dsot_runtime::{Config, infra::init_config_builder, init};

#[tokio::main]
async fn main() {
    music_brainz::init_user_agent("dsot", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let config = init_config_builder(true)
        .expect("Failed to initialize config builder")
        .build();

    let mut config = Config::from_value(config);
    if let Some(lg) = config.logger.as_mut() {
        lg.enabled = true;
        lg.use_console = true;
        lg.use_file = false;
        lg.level = "trace".to_string();
    }

    let runtime = init(config).await.expect("Failed to initialize runtime");

    runtime.shutdown();
}
