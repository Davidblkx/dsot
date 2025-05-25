use super::{MusicFile, MusicFileFormat};

impl MusicFile {
    pub fn new(storage: &uuid::Uuid, path: &str) -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            storage_id: *storage,
            path: path.to_string(),
            format: 0,
            size: 0,
            recording_id: None,
            chromaprint: None,
            need_better: false,
        }
    }

    pub fn set_format(&mut self, format: MusicFileFormat) {
        self.format = format.to_u32();
    }

    pub fn get_format(&self) -> MusicFileFormat {
        MusicFileFormat::from_u32(self.format)
    }
}

impl MusicFileFormat {
    pub fn from_u32(value: u32) -> Self {
        match value {
            1 => MusicFileFormat::MP3,
            2 => MusicFileFormat::FLAC,
            3 => MusicFileFormat::WAV,
            4 => MusicFileFormat::OGG,
            5 => MusicFileFormat::AAC,
            6 => MusicFileFormat::WMA,
            _ => MusicFileFormat::Unknown,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            MusicFileFormat::MP3 => 1,
            MusicFileFormat::FLAC => 2,
            MusicFileFormat::WAV => 3,
            MusicFileFormat::OGG => 4,
            MusicFileFormat::AAC => 5,
            MusicFileFormat::WMA => 6,
            MusicFileFormat::Unknown => 0,
        }
    }
}

impl From<MusicFileFormat> for u32 {
    fn from(format: MusicFileFormat) -> Self {
        format.to_u32()
    }
}

impl From<u32> for MusicFileFormat {
    fn from(value: u32) -> Self {
        MusicFileFormat::from_u32(value)
    }
}

impl PartialEq for MusicFileFormat {
    fn eq(&self, other: &Self) -> bool {
        self.to_u32() == other.to_u32()
    }
}
