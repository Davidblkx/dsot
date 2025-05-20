use super::Track;

impl Track {
    pub fn new(title: String, release_media_id: &uuid::Uuid, recording_id: &uuid::Uuid) -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            release_media_id: *release_media_id,
            media_index: 0,
            release_index: 0,
            track_number: 0,
            position: None,
            title,
            mbid: None,
            recording_id: *recording_id,
        }
    }
}
