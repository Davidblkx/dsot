#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct NetworkConfig {
    pub persist_key: bool,
    pub key_file: Option<String>,
    pub public_name: Option<String>,
    pub public_desc: Option<String>,
    pub lazy: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            persist_key: true,
            key_file: None,
            public_name: None,
            public_desc: None,
            lazy: false,
        }
    }
}
