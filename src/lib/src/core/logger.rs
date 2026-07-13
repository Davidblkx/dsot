use std::{path::PathBuf, time::SystemTime};

use super::{config::DsotAppConfig, init::DsotCoreInitOptions};
use crate::error::Result;

impl DsotCoreInitOptions {
    pub fn init_debug_logger(&self) -> Result<bool> {
        if !self.debug {
            return Ok(false);
        }

        let file = if self.cap.can_disk_access() {
            let date_now = chrono::Local::now().format("%Y_%m_%d_%H_%M").to_string();
            match sysdirs::temp_dir() {
                Some(p) => Some(p.join(format!("dsot_logs.{}.txt", date_now))),
                None => None,
            }
        } else {
            None
        };

        init_logger(log::LevelFilter::Debug, file)?;

        Ok(true)
    }

    pub fn init_logger_from_config(&self, config: &DsotAppConfig) -> Result<()> {
        let level = config
            .value
            .log_level
            .as_ref()
            .map_or(::log::LevelFilter::Info, |l| {
                match l.to_uppercase().as_str() {
                    "TRACE" => ::log::LevelFilter::Trace,
                    "DEBUG" => ::log::LevelFilter::Debug,
                    "INFO" => ::log::LevelFilter::Info,
                    "WARN" => ::log::LevelFilter::Warn,
                    "ERROR" => ::log::LevelFilter::Error,
                    _ => log::LevelFilter::Info,
                }
            });

        init_logger(level, config.value.log_file.clone())?;

        Ok(())
    }
}

fn init_logger(
    level: log::LevelFilter,
    file: Option<PathBuf>,
) -> std::result::Result<(), fern::InitError> {
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
        .level(level)
        .chain(std::io::stdout());

    if let Some(file_path) = file {
        dispatch = dispatch.chain(fern::log_file(file_path)?);
    }

    dispatch.apply()?;

    Ok(())
}
