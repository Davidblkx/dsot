use bakunin_config::{Value};
use crate::Config;

impl Config {
    pub fn read_raw_config(&self, path: &str) -> Option<Value> {
        if path.is_empty() {
            return self.raw.clone();
        }

        if let Some(raw) = &self.raw {
            let mut val = raw.clone();
            for part in path.split('.') {
                val = val.get(part);
            }

            return Some(val);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bakunin_config::{value_map, Value};

    #[test]
    fn when_none_return_none() {
        let cfg = Config {
            data_location: std::path::PathBuf::from("./"),
            logger: None,
            user: "test_user".to_string(),
            raw: None,
        };

        let result = cfg.read_raw_config("some.path");
        assert!(result.is_none());
    }

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
            raw: Some(raw_value.clone()),
        };

        let result = cfg.read_raw_config("");
        assert_eq!(result, Some(raw_value));
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
        let cfg = Config::from_value(raw_value.clone());

        let result = cfg.read_raw_config("other.second.third").unwrap();
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
        let cfg = Config::from_value(raw_value.clone());

        let result = cfg.read_raw_config("other.second.thirdss").unwrap();
        assert_eq!(result, Value::None);
    }
}
