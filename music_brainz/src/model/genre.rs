#[derive(Clone, Debug, serde::Deserialize)]
pub struct Genre {
    pub name: String,
    pub count: u32,
    pub disambiguation: Option<String>,
    pub id: String,
}
