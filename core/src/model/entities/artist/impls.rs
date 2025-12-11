use uuid::Uuid;

use music_brainz::model::artist::ArtistType;

impl super::Artist {
    /// Returns the musicbrainz artist type for this artist.
    pub fn get_artist_type(&self) -> ArtistType {
        ArtistType::from_u32(self.artist_type_id)
    }

    /// Allows to set the musicbrainz artist type for this artist.
    pub fn set_artist_type(&mut self, artist_type: ArtistType) {
        self.artist_type_id = artist_type.to_u32();
    }

    /// Creates a new artist with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::now_v7(),
            mbid: None,
            name: name.to_string(),
            sort_name: None,
            artist_type_id: 1,
        }
    }

    pub fn set_sort_name<T: Into<String>>(mut self, sort_name: T) -> Self {
        self.sort_name = Some(sort_name.into());
        self
    }
}
