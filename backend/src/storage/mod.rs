pub mod redb;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestDataV0 {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: String,
}
