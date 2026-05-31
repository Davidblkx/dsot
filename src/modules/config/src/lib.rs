use std::path::PathBuf;

use bakunin_config::{BakuninConfig, Value};

mod error;
mod loader;
mod options;

pub use error::{DsotConfigError, Result};
pub use options::ConfigOptions;

#[derive(Debug)]
pub struct DsotConfig<T> {
    pub data_dir: PathBuf,
    pub value: T,
    pub inner: Value,
    pub handler: BakuninConfig,
}

impl<T> DsotConfig<T> {
    pub fn get(&self, path: &str) -> Value {
        let mut parts = path.split(".");

        let mut val = match parts.next() {
            Some(v) => self.inner.get(v),
            _ => Value::None,
        };

        for p in parts {
            val = val.get(p);
        }

        val.clone()
    }
}

#[cfg(test)]
mod tests {
    use bakunin_config::value_map;

    use super::*;

    #[test]
    fn can_get_by_path() {
        let target = DsotConfig {
            data_dir: PathBuf::new(),
            handler: BakuninConfig::new(),
            value: 0,
            inner: value_map! {
                p1: value_map! {
                    p2: 10
                }
            },
        };

        let n = target.get("p1.p2");
        assert_eq!(n, Value::Integer(10));
    }

    #[test]
    fn dont_fail_when_null() {
        let target = DsotConfig {
            data_dir: PathBuf::new(),
            handler: BakuninConfig::new(),
            inner: Value::None,
            value: 10,
        };

        let n = target.get("p1.p2");
        assert_eq!(n, Value::None);
    }
}
