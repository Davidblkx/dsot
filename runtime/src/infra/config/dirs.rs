pub fn get_data_location(v: &bakunin_config::Value) -> String {
    match v.get("data_location").try_into_string() {
        Ok(path) => path,
        Err(_) => {
            match dirs::home_dir() {
                Some(home) => format!("{}/.dsot", home.to_string_lossy()),
                None => "./".to_string(),
            }
        },
    }
}
