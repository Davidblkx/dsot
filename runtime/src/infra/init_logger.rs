use crate::infra::config::logger::LogConfig;
use flexi_logger::{
    Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, LoggerHandle, Naming, WriteMode,
};

static HAS_LOGGER: std::sync::Once = std::sync::Once::new();

fn get_log_level<'a>(cfg: &'a LogConfig) -> &'a str {
    if cfg.use_file {
        cfg.file_level.as_deref().unwrap_or(cfg.level.as_str())
    } else {
        cfg.console_level.as_deref().unwrap_or(cfg.level.as_str())
    }
}

fn get_duplicate_level(cfg: &LogConfig) -> Duplicate {
    let level = cfg.console_level.as_deref().unwrap_or(cfg.level.as_str());
    match level {
        "trace" => Duplicate::Trace,
        "debug" => Duplicate::Debug,
        "info" => Duplicate::Info,
        "warn" => Duplicate::Warn,
        "error" => Duplicate::Error,
        _ => Duplicate::All, // Default to All if level is not recognized
    }
}

fn setup_console_logger(cfg: &LogConfig, lg: Logger) -> Logger {
    if cfg.to_stderr {
        lg.log_to_stderr()
    } else {
        lg.log_to_stdout()
    }
}

fn setup_file_logger(cfg: &LogConfig, lg: Logger) -> Logger {
    let lg = lg
        .log_to_file(
            FileSpec::default()
                .directory(&cfg.to_folder)
                .basename("dsot"),
        )
        .write_mode(WriteMode::Async)
        .rotate(
            Criterion::AgeOrSize(Age::Day, 100 * 1024 * 1024), // Rotate daily or when file size exceeds 100MB
            Naming::Timestamps,
            Cleanup::KeepLogFiles(15), // Keep logs for 15 days
        );

    if cfg.use_console {
        if cfg.to_stderr {
            return lg.duplicate_to_stderr(get_duplicate_level(cfg));
        } else {
            return lg.duplicate_to_stdout(get_duplicate_level(cfg));
        }
    }

    lg
}

/// Initializes the runtime logger based on the provided configuration.
/// # Arguments
/// * `cfg` - A reference to the `LogConfig` containing the logger settings.
///
/// # Returns
/// An `Option<LoggerHandle>`, which is `Some` if the logger was successfully initialized,
/// or `None` if logging is disabled or already initialized.
///
/// # Panics
/// If the logger fails to initialize or start, it will panic with an error message.
///
/// # Notes
/// This function uses a `Once` instance to ensure that the logger is initialized only once,
pub fn init_runtime_logger(cfg: &LogConfig) -> Option<LoggerHandle> {
    if HAS_LOGGER.is_completed() {
        log::trace!("Logger has already been initialized. Skipping initialization.");
        return None;
    }

    HAS_LOGGER.call_once(|| {
        log::trace!("Logger initialization started.");
    });

    if !cfg.enabled || (!cfg.use_console && !cfg.use_file) {
        return None;
    }

    let logger = match Logger::try_with_str(get_log_level(cfg)) {
        Ok(logger) => logger,
        Err(e) => {
            panic!("Failed to initialize logger: {}", e);
        }
    };

    let logger = if cfg.use_file {
        setup_file_logger(cfg, logger)
    } else {
        setup_console_logger(cfg, logger)
    };

    let handler = match logger.start() {
        Ok(handle) => handle,
        Err(e) => {
            panic!("Failed to start logger: {}", e);
        }
    };

    return Some(handler);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::infra::config::logger::LogConfig;

    #[test]
    fn test_init_runtime_logger() {
        let log_config = LogConfig {
            enabled: true,
            use_console: true,
            use_file: false,
            to_stderr: true,
            to_folder: PathBuf::from("/tmp"),
            level: String::from("info"),
            file_level: None,
            console_level: None,
        };

        let logger_handle = init_runtime_logger(&log_config);
        assert!(logger_handle.is_some());

        let second_handle = init_runtime_logger(&log_config);
        assert!(
            second_handle.is_none(),
            "Logger should not be initialized again"
        );
    }

    #[test]
    fn test_init_runtime_logger_disabled() {
        let log_config = LogConfig {
            enabled: false,
            use_console: false,
            use_file: false,
            to_stderr: false,
            to_folder: PathBuf::from("/tmp"),
            level: String::from("info"),
            file_level: None,
            console_level: None,
        };

        let logger_handle = init_runtime_logger(&log_config);
        assert!(logger_handle.is_none());
    }
}
