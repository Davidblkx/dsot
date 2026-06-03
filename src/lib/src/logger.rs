use std::{path::PathBuf, time::SystemTime};

#[derive(Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<&str> for LogLevel {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "TRACE" => LogLevel::Trace,
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            _ => LogLevel::Info,
        }
    }
}

impl From<&Option<String>> for LogLevel {
    fn from(value: &Option<String>) -> Self {
        match value {
            Some(v) => v.as_str().into(),
            None => LogLevel::Info,
        }
    }
}

impl Into<log::LevelFilter> for LogLevel {
    fn into(self) -> log::LevelFilter {
        match self {
            LogLevel::Trace => log::LevelFilter::Trace,
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Error => log::LevelFilter::Error,
        }
    }
}

pub fn init_log(level: LogLevel, file: Option<PathBuf>) -> Result<(), fern::InitError> {
    let mut dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level.into())
        .chain(std::io::stdout());

    if let Some(file_path) = file {
        dispatch = dispatch.chain(fern::log_file(file_path)?);
    }

    dispatch.apply()?;

    Ok(())
}
