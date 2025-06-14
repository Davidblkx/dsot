pub mod error;
pub mod infra;
pub mod config;

pub use infra::Config;
use infra::{init_folder, init_runtime_logger};

pub struct Runtime {
    pub config: Config,
    pub version: &'static str,
    pub sqlite_pool: sqlx::SqlitePool,

    logger_handler: Option<flexi_logger::LoggerHandle>,
}

impl Runtime {
    pub fn shutdown(&self) {
        log::debug!("Exiting runtime version: {}", self.version);
        if let Some(logger) = &self.logger_handler {
            logger.shutdown();
        }
    }
}

pub async fn init(config: Config) -> error::Result<Runtime> {
    // Initialize the logger with the provided configuration
    let logger_handler = match &config.logger {
        Some(log_config) => init_runtime_logger(log_config),
        None => None,
    };

    log::debug!(
        "Initializing runtime using data folder: {}",
        config.data_location.display()
    );
    init_folder(&config.data_location)?;

    let sqlite_pool = infra::db::initialize_database(&config).await?;

    // Initialize the runtime with the provided configuration
    let runtime = Runtime {
        config,
        version: env!("CARGO_PKG_VERSION"),
        logger_handler,
        sqlite_pool,
    };

    // Return the initialized runtime
    Ok(runtime)
}
