use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "t", content = "v")]
pub enum UpdateValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

pub trait IntoUpdateValue {
    fn into_update_value(&self) -> UpdateValue;
}

impl UpdateValue {
    pub fn get_if_diff<T>(v1: &T, v2: &T) -> Option<UpdateValue>
    where
        T: PartialEq + IntoUpdateValue + ?Sized,
    {
        if v1 != v2 {
            Some(v2.into_update_value())
        } else {
            None
        }
    }

    pub fn from_utc_now() -> UpdateValue {
        ::chrono::Utc::now().into_update_value()
    }
}

impl<T: IntoUpdateValue> IntoUpdateValue for Option<T> {
    fn into_update_value(&self) -> UpdateValue {
        match self {
            Some(value) => value.into_update_value(),
            None => UpdateValue::Null,
        }
    }
}

impl IntoUpdateValue for String {
    fn into_update_value(&self) -> UpdateValue {
        UpdateValue::Text(self.clone())
    }
}

impl IntoUpdateValue for i64 {
    fn into_update_value(&self) -> UpdateValue {
        UpdateValue::Integer(*self)
    }
}

impl IntoUpdateValue for f64 {
    fn into_update_value(&self) -> UpdateValue {
        UpdateValue::Real(*self)
    }
}

impl IntoUpdateValue for bool {
    fn into_update_value(&self) -> UpdateValue {
        UpdateValue::Integer(*self as i64)
    }
}

impl IntoUpdateValue for ::chrono::DateTime<::chrono::Utc> {
    fn into_update_value(&self) -> UpdateValue {
        UpdateValue::Text(self.to_rfc3339())
    }
}

impl IntoUpdateValue for ::chrono::NaiveDate {
    fn into_update_value(&self) -> UpdateValue {
        UpdateValue::Text(self.to_string())
    }
}

impl IntoUpdateValue for ::uuid::Uuid {
    fn into_update_value(&self) -> UpdateValue {
        UpdateValue::Blob(self.as_bytes().to_vec())
    }
}

impl IntoUpdateValue for ::sqlx::types::Json<Vec<String>> {
    fn into_update_value(&self) -> UpdateValue {
        let raw_json = match serde_json::to_string(self) {
            Ok(json) => json,
            Err(_) => "[]".to_string(),
        };
        UpdateValue::Text(raw_json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_detect_diff_string() {
        let diff = UpdateValue::get_if_diff(&"v1".to_string(), &"v2".to_string());
        assert_eq!(Some(UpdateValue::Text("v2".to_string())), diff);
    }

    #[test]
    fn can_detect_diff_option_string() {
        let diff = UpdateValue::get_if_diff(&Some("v1".to_string()), &Some("v2".to_string()));
        assert_eq!(Some(UpdateValue::Text("v2".to_string())), diff);
    }

    #[test]
    fn can_map_json() {
        let items = vec!["a".to_string(), "b".to_string()];
        let json: sqlx::types::Json<Vec<String>> = items.into();
        let target = json.into_update_value();
        assert_eq!(UpdateValue::Text(r#"["a","b"]"#.to_string()), target);
    }
}
