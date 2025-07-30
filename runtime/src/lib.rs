mod model;

pub mod error;
pub mod infra;

pub use infra::Config;
use infra::{init_folder, init_runtime_logger};

use crate::infra::db::DatabaseHandler;

pub use model::*;

pub struct Runtime {
    pub config: Config,
    pub version: &'static str,
    pub db: DatabaseHandler,

    pub(crate) logger_handler: Option<flexi_logger::LoggerHandle>,
}

impl Runtime {
    pub fn shutdown(&self, exit_code: usize) {
        if exit_code == 0 {
            log::debug!("Exiting runtime version: {}", self.version);
        } else {
            log::debug!(
                "Exiting runtime version: {} with error code: {}",
                self.version,
                exit_code
            );
        }

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

    let db = infra::db::DatabaseHandler::new(&config).await?;

    // Initialize the runtime with the provided configuration
    let runtime = Runtime {
        config,
        version: env!("CARGO_PKG_VERSION"),
        logger_handler,
        db,
    };

    // Return the initialized runtime
    Ok(runtime)
}
