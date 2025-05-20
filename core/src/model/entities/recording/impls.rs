use super::Recording;

impl Recording {
    pub fn new(title: String) -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            mbid: None,
            title,
            length: None,
            isrc: None,
            work_id: None,
            year: None,
            disambiguation: None,
        }
    }
}
