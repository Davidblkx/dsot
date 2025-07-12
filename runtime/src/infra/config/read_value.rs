use crate::Config;
use bakunin_config::Value;

impl Config {
    pub fn get_config_value(&self, path: &str) -> Value {
        let mut val = self.raw.clone();
        if path.is_empty() {
            return val;
        }
        for part in path.split('.') {
            val = val.get(part);
        }
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bakunin_config::{BakuninConfig, Value, value_map};

    #[test]
    fn when_empty_path_return_raw() {
        let raw_value = value_map! {
            some: "value",
            path: "test"
        };
        let cfg = Config {
            data_location: std::path::PathBuf::from("./"),
            logger: None,
            user: "test_user".to_string(),
            raw: raw_value.clone(),
            handler: bakunin_config::BakuninConfig::new(),
        };

        let result = cfg.get_config_value("");
        assert_eq!(result, raw_value);
    }

    #[test]
    fn when_valid_path_return_value() {
        let raw_value = value_map! {
            some: "value",
            other: value_map! {
                some: "nested_value",
                second: value_map! {
                    third: "deep_value"
                }
            }
        };
        let handler = BakuninConfig::new().with_memory_layer("root", raw_value);
        let cfg = Config::from_handler(handler).unwrap();

        let result = cfg.get_config_value("other.second.third");
        assert_eq!(result, Value::String("deep_value".into()));
    }

    #[test]
    fn when_not_valid_path_return_value_none() {
        let raw_value = value_map! {
            some: "value",
            other: value_map! {
                some: "nested_value",
                second: value_map! {
                    third: "deep_value"
                }
            }
        };
        let handler = BakuninConfig::new().with_memory_layer("root", raw_value);
        let cfg = Config::from_handler(handler).unwrap();

        let result = cfg.get_config_value("other.second.thirdss");
        assert_eq!(result, Value::None);
    }
}
