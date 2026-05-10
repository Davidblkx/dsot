use super::ReleaseTrack;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ReleaseMedia {
    /// The title of the media.
    pub title: Option<String>,
    /// The position of the media in the release.
    pub position: Option<i32>,
    /// The format of the media.
    pub format: Option<String>,
    /// The track count of the media.
    #[serde(alias = "track-count")]
    pub track_count: Option<i32>,
    /// The list of tracks in the media.
    pub tracks: Option<Vec<ReleaseTrack>>,
}
