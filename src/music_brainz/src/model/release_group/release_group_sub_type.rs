#[derive(Debug, Clone, serde::Deserialize)]
pub enum ReleaseGroupSubType {
    Compilation,
    Soundtrack,
    Spokenword,
    Interview,
    Audiobook,
    AudioDrama,
    Live,
    Remix,
    DjMix,
    MixtapeStreet,
    Demo,
    FieldRecording,
    Unknown
}

impl std::fmt::Display for ReleaseGroupSubType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReleaseGroupSubType::Compilation => write!(f, "Compilation"),
            ReleaseGroupSubType::Soundtrack => write!(f, "Soundtrack"),
            ReleaseGroupSubType::Spokenword => write!(f, "Spokenword"),
            ReleaseGroupSubType::Interview => write!(f, "Interview"),
            ReleaseGroupSubType::Audiobook => write!(f, "Audiobook"),
            ReleaseGroupSubType::AudioDrama => write!(f, "Audio Drama"),
            ReleaseGroupSubType::Live => write!(f, "Live"),
            ReleaseGroupSubType::Remix => write!(f, "Remix"),
            ReleaseGroupSubType::DjMix => write!(f, "DJ-Mix"),
            ReleaseGroupSubType::MixtapeStreet => write!(f, "Mixtape/Street"),
            ReleaseGroupSubType::Demo => write!(f, "Demo"),
            ReleaseGroupSubType::FieldRecording => write!(f, "Field recording"),
            ReleaseGroupSubType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Default for ReleaseGroupSubType {
    fn default() -> Self {
        ReleaseGroupSubType::Unknown
    }
}
