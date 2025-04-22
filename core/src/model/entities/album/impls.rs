use uuid::Uuid;

impl super::Album {
    /// Creates a new album with the given title and year.
    pub fn new(title: &str, year: i16) -> Self {
        Self {
            id: Uuid::now_v7(),
            mbid: None,
            title: title.to_string(),
            year: if year > 0 {
                Some(year)
            } else {
                None
            },
        }
    }
}
