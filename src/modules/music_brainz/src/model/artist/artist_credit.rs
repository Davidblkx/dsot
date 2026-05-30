#[derive(Clone, Debug, serde::Deserialize)]
pub struct ArtistCredit {
    pub name: String,
    pub joinphrase: Option<String>,
    pub artist: Option<crate::entities::Artist>,
}
