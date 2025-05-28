static DEFAULT_LOG_LEVEL: &'static str = "warn";

/// Configuration for logging in the runtime environment.
pub struct LogConfig {
    /// Defines if logs are enabled. If false, no logs will be written.
    pub enabled: bool,
    /// The logging level, e.g., "info", "warn", "error".
    pub level: String,
    /// Whether to log to a file.
    pub use_file: bool,
    /// Whether to log to the console.
    pub use_console: bool,
    /// Override the logging level for file output, if used.
    pub file_level: Option<String>,
    /// Override the logging level for console output, if used.
    pub console_level: Option<String>,
    /// The path to the log file, if logging to a file.
    pub to_folder: String,
    /// Whether to log to stderr instead of stdout.
    pub to_stderr: bool,
}

fn parse_log_level(value: String) -> String {
    let v = value.to_lowercase();
    if v == "debug" || v == "info" || v == "warn" || v == "error" || v == "trace" {
        return v;
    }

    return DEFAULT_LOG_LEVEL.to_string();
}

fn get_log_level(v: &bakunin_config::Value) -> String {
    let value = std::env::var("DSOT_LOG_LEVEL").ok()
        .or(v.get("level").try_into_string().ok());

    match value {
        Some(level) => parse_log_level(level),
        None => DEFAULT_LOG_LEVEL.to_string(),
    }
}

fn get_log_path(v: &bakunin_config::Value, data_location: &str) -> String {
    match v.get("to_file_path").try_into_string() {
        Ok(path) => path,
        Err(_) => format!("{}/logs", data_location),
    }
}

impl LogConfig {
    pub fn from_value(v: bakunin_config::Value, data_location: &str) -> Self {
        LogConfig {
            enabled: v.get("enabled").into_bool_or(true),
            level: get_log_level(&v),
            use_file: v.get("use_file").into_bool_or(true),
            use_console: v.get("use_console").into_bool_or(false),
            file_level: v.get("file_level").try_into_string().ok().map(parse_log_level),
            console_level: v.get("console_level").try_into_string().ok().map(parse_log_level),
            to_folder: get_log_path(&v, data_location),
            to_stderr: v.get("to_stderr").into_bool_or(true),
        }
    }
}
