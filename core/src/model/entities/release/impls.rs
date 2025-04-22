use uuid::Uuid;

use super::Release;
use crate::model::entities::album::Album;

use music_brainz::model::release::ReleaseStatus;

impl Release {
    pub fn new(title: &str, album_id: &Uuid) -> Self {
        Self {
            id: Uuid::now_v7(),
            title: title.to_string(),
            country: None,
            duration: None,
            format: None,
            mbid: None,
            status: None,
            track_count: None,
            year: None,
            album_id: *album_id,
        }
    }

    pub fn for_album(album: &Album) -> Self {
        Self {
            id: Uuid::now_v7(),
            title: album.title.clone(),
            country: None,
            duration: None,
            format: None,
            mbid: None,
            status: None,
            track_count: None,
            year: None,
            album_id: album.id,
        }
    }

    pub fn get_status(&self) -> ReleaseStatus {
        self.status.into()
    }

    pub fn set_status(&mut self, status: ReleaseStatus) {
        self.status = Some(status.to_u32());
    }
}
