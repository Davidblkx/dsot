pub mod infra;
pub mod error;

pub use infra::Config;

pub struct Runtime {
    pub config: Config,
    pub version: &'static str,
}

impl Runtime {

}

pub async fn init(config: Config) -> error::Result<Runtime> {
    log::debug!("Initializing runtime using data folder: {:?}", config.data_location);

    // Initialize the runtime with the provided configuration
    let runtime = Runtime {
        config,
        version: env!("CARGO_PKG_VERSION"),
    };

    // Perform any necessary setup here, such as initializing databases or services
    // For example, you might want to set up a database connection pool or initialize logging

    // Return the initialized runtime
    Ok(runtime)
}
