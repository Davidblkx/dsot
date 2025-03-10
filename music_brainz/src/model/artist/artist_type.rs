
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

impl ArtistType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Person" => ArtistType::Person,
            "Group" => ArtistType::Group,
            "Orchestra" => ArtistType::Orchestra,
            "Choir" => ArtistType::Choir,
            "Character" => ArtistType::Character,
            "Other" => ArtistType::Other,
            _ => ArtistType::Unknown,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ArtistType::Person => "Person",
            ArtistType::Group => "Group",
            ArtistType::Orchestra => "Orchestra",
            ArtistType::Choir => "Choir",
            ArtistType::Character => "Character",
            ArtistType::Other => "Other",
            ArtistType::Unknown => "Unknown",
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            ArtistType::Person => 1,
            ArtistType::Group => 2,
            ArtistType::Orchestra => 3,
            ArtistType::Choir => 4,
            ArtistType::Character => 5,
            ArtistType::Other => 6,
            ArtistType::Unknown => 0,
        }
    }

    pub fn from_u32(u: u32) -> Self {
        match u {
            1 => ArtistType::Person,
            2 => ArtistType::Group,
            3 => ArtistType::Orchestra,
            4 => ArtistType::Choir,
            5 => ArtistType::Character,
            6 => ArtistType::Other,
            _ => ArtistType::Unknown,
        }
    }
}

impl From<u32> for ArtistType {
    fn from(u: u32) -> Self {
        ArtistType::from_u32(u)
    }
}

impl From<&str> for ArtistType {
    fn from(s: &str) -> Self {
        ArtistType::from_str(s)
    }
}

impl From<ArtistType> for u32 {
    fn from(a: ArtistType) -> u32 {
        a.to_u32()
    }
}

impl From<ArtistType> for String {
    fn from(a: ArtistType) -> String {
        a.to_str().to_string()
    }
}

impl std::fmt::Display for ArtistType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Default for ArtistType {
    fn default() -> Self {
        ArtistType::Unknown
    }
}
