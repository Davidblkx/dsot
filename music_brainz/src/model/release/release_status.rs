#[derive(Debug, Clone, serde::Deserialize)]
pub enum ReleaseStatus {
    Official,
    Promotion,
    Bootleg,
    PseudoRelease,
    Withdrawn,
    Expunged,
    Cancelled,
    Unknown,
}

impl ReleaseStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Official" => ReleaseStatus::Official,
            "Promotion" => ReleaseStatus::Promotion,
            "Bootleg" => ReleaseStatus::Bootleg,
            "PseudoRelease" => ReleaseStatus::PseudoRelease,
            "Withdrawn" => ReleaseStatus::Withdrawn,
            "Expunged" => ReleaseStatus::Expunged,
            "Cancelled" => ReleaseStatus::Cancelled,
            _ => ReleaseStatus::Unknown,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ReleaseStatus::Official => "Official",
            ReleaseStatus::Promotion => "Promotion",
            ReleaseStatus::Bootleg => "Bootleg",
            ReleaseStatus::PseudoRelease => "PseudoRelease",
            ReleaseStatus::Withdrawn => "Withdrawn",
            ReleaseStatus::Expunged => "Expunged",
            ReleaseStatus::Cancelled => "Cancelled",
            ReleaseStatus::Unknown => "Unknown",
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            ReleaseStatus::Official => 1,
            ReleaseStatus::Promotion => 2,
            ReleaseStatus::Bootleg => 3,
            ReleaseStatus::PseudoRelease => 4,
            ReleaseStatus::Withdrawn => 5,
            ReleaseStatus::Expunged => 6,
            ReleaseStatus::Cancelled => 7,
            ReleaseStatus::Unknown => 0,
        }
    }

    pub fn from_u32(u: u32) -> Self {
        match u {
            1 => ReleaseStatus::Official,
            2 => ReleaseStatus::Promotion,
            3 => ReleaseStatus::Bootleg,
            4 => ReleaseStatus::PseudoRelease,
            5 => ReleaseStatus::Withdrawn,
            6 => ReleaseStatus::Expunged,
            7 => ReleaseStatus::Cancelled,
            _ => ReleaseStatus::Unknown,
        }
    }
}

impl std::fmt::Display for ReleaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl From<u32> for ReleaseStatus {
    fn from(u: u32) -> Self {
        ReleaseStatus::from_u32(u)
    }
}

impl From<Option<u32>> for ReleaseStatus {
    fn from(u: Option<u32>) -> Self {
        match u {
            Some(u) => ReleaseStatus::from_u32(u),
            None => ReleaseStatus::Unknown,
        }
    }
}

impl From<&str> for ReleaseStatus {
    fn from(s: &str) -> Self {
        ReleaseStatus::from_str(s)
    }
}

impl From<ReleaseStatus> for u32 {
    fn from(a: ReleaseStatus) -> u32 {
        a.to_u32()
    }
}

impl From<ReleaseStatus> for String {
    fn from(a: ReleaseStatus) -> String {
        a.to_str().to_string()
    }
}
