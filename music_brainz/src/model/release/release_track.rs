use crate::entities::recording::Recording;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ReleaseTrack {
    pub id: String,
    pub title: Option<String>,
    pub position: Option<i32>,
    pub length: Option<u64>,
    pub number: Option<String>,
    pub recording: Option<Recording>,
}
