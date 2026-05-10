#[derive(Clone, Debug, serde::Deserialize)]
pub struct AreaAlias {
    pub locale: Option<String>,
    pub name: String,
    pub primary: Option<bool>,
    #[serde(alias = "sort-name")]
    pub sort_name: Option<String>,
    pub r#type: Option<String>,
    pub begin: Option<String>,
    pub end: Option<String>,
    pub ended: Option<bool>
}
