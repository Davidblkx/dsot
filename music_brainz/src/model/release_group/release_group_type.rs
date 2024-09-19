#[derive(Debug, Clone, serde::Deserialize)]
pub enum ReleaseGroupType {
    Album,
    Single,
    EP,
    Broadcast,
    Other,
    Unknown,
}

impl std::fmt::Display for ReleaseGroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReleaseGroupType::Album => write!(f, "Album"),
            ReleaseGroupType::Single => write!(f, "Single"),
            ReleaseGroupType::EP => write!(f, "EP"),
            ReleaseGroupType::Broadcast => write!(f, "Broadcast"),
            ReleaseGroupType::Other => write!(f, "Other"),
            ReleaseGroupType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Default for ReleaseGroupType {
    fn default() -> Self {
        ReleaseGroupType::Unknown
    }
}
