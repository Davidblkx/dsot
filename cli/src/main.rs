mod config;

use dsot_runtime::init;

#[tokio::main]
async fn main() {
    env_logger::init();
    music_brainz::init_user_agent("dsot", env!("CARGO_PKG_VERSION"), "dev@davidpires.pt").unwrap();

    let value = config::load_config();
    let config = dsot_runtime::infra::Config::from_value(value);

    let runtime = init(config).await;
    match runtime {
        Ok(runtime) => {
            log::info!("Runtime initialized successfully with version: {:?}", runtime.version);
        }
        Err(e) => {
            log::error!("Failed to initialize runtime: {}", e);
            std::process::exit(1);
        }
    }
}
