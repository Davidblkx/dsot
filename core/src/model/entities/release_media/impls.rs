use uuid::Uuid;
use super::{ReleaseMedia, ReleaseMediaFormat};

impl ReleaseMediaFormat {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => ReleaseMediaFormat::Unknown,
            1 => ReleaseMediaFormat::CD,
            2 => ReleaseMediaFormat::Vinyl,
            3 => ReleaseMediaFormat::Digital,
            4 => ReleaseMediaFormat::Cassette,
            _ => ReleaseMediaFormat::Unknown,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            ReleaseMediaFormat::Unknown => 0,
            ReleaseMediaFormat::CD => 1,
            ReleaseMediaFormat::Vinyl => 2,
            ReleaseMediaFormat::Digital => 3,
            ReleaseMediaFormat::Cassette => 4,
        }
    }
}

impl From<u32> for ReleaseMediaFormat {
    fn from(value: u32) -> Self {
        ReleaseMediaFormat::from_u32(value)
    }
}

impl From<ReleaseMediaFormat> for u32 {
    fn from(value: ReleaseMediaFormat) -> Self {
        value.to_u32()
    }
}

impl ReleaseMedia {
    pub fn new(release_id: &Uuid, format: &ReleaseMediaFormat) -> Self {
        Self {
            id: Uuid::now_v7(),
            release_id: *release_id,
            format: format.to_u32(),
            count: 0,
            position: 0,
            mbid: None,
        }
    }

    pub fn get_format(&self) -> ReleaseMediaFormat {
        ReleaseMediaFormat::from_u32(self.format)
    }

    pub fn set_format(&mut self, format: ReleaseMediaFormat) {
        self.format = format.to_u32();
    }
}
