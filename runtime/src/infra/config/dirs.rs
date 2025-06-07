use std::path::PathBuf;

pub fn get_data_location(v: &bakunin_config::Value) -> PathBuf {
    match v.get("data_location").try_into_string() {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            match dirs::home_dir() {
                Some(home) => home.join(".dsot"),
                None => PathBuf::from("./"),
            }
        },
    }
}
