use super::Inbox;
impl Inbox {
    pub fn new() -> Self {
        Inbox {
            id: uuid::Uuid::now_v7(),
            title: None,
            artist: None,
            album: None,
            file: None,
            extra_info: None,
        }
    }

    pub fn set_title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn set_artist<T: Into<String>>(mut self, artist: T) -> Self {
        self.artist = Some(artist.into());
        self
    }

    pub fn set_album<T: Into<String>>(mut self, album: T) -> Self {
        self.album = Some(album.into());
        self
    }

    pub fn set_file<T: Into<String>>(mut self, file: T) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn set_extra_info<T: Into<String>>(mut self, extra_info: T) -> Self {
        self.extra_info = Some(extra_info.into());
        self
    }
}
