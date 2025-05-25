use super::Work;

impl Work {
    pub fn new(title: String) -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            mbid: None,
            title,
            kind: None,
            disambiguation: None,
            language: None,
        }
    }
}
