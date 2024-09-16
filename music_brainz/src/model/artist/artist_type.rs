
#[derive(Debug, Clone, serde::Deserialize)]
pub enum ArtistType {
    Person,
    Group,
    Orchestra,
    Choir,
    Character,
    Other,
    Unknown,
}

impl std::fmt::Display for ArtistType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArtistType::Person => write!(f, "Person"),
            ArtistType::Group => write!(f, "Group"),
            ArtistType::Orchestra => write!(f, "Orchestra"),
            ArtistType::Choir => write!(f, "Choir"),
            ArtistType::Character => write!(f, "Character"),
            ArtistType::Other => write!(f, "Other"),
            ArtistType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Default for ArtistType {
    fn default() -> Self {
        ArtistType::Unknown
    }
}
