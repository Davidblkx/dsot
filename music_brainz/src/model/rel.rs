use crate::entities::{Work, Recording, Artist};

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Relationship {
    pub work: Option<Work>,
    pub recording: Option<Recording>,
    pub r#type: Option<String>,
    pub artist: Option<Artist>
}
