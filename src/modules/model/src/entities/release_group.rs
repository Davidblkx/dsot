use std::borrow::Cow;

use dsot_db_sync::model::{IntoUpdateValue, UpdateValue};
use dsot_derive::SyncEntity;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, Sqlite, Type, sqlite::SqliteTypeInfo};
use uuid::Uuid;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum ReleaseGroupType {
    Album,
    Single,
    EP,
    Broadcast,
    Live,
    Other,
    #[default]
    Unknown,
    /// Fallback that captures any string the database produced which doesn't
    /// match a known variant — legacy rows, future variants this build hasn't
    /// learned about yet, or hand-edited values. Decoding never fails.
    /// Note: a `Custom("Album")` written to the DB round-trips as `Album`.
    Custom(String),
}

impl ReleaseGroupType {
    pub fn as_db_str(&self) -> Cow<'_, str> {
        match self {
            Self::Album => Cow::Borrowed("Album"),
            Self::Single => Cow::Borrowed("Single"),
            Self::EP => Cow::Borrowed("EP"),
            Self::Broadcast => Cow::Borrowed("Broadcast"),
            Self::Live => Cow::Borrowed("Live"),
            Self::Other => Cow::Borrowed("Other"),
            Self::Unknown => Cow::Borrowed("Unknown"),
            Self::Custom(s) => Cow::Borrowed(s.as_str()),
        }
    }

    pub fn from_db_str(s: &str) -> Self {
        match s {
            "Album" => Self::Album,
            "Single" => Self::Single,
            "EP" => Self::EP,
            "Broadcast" => Self::Broadcast,
            "Live" => Self::Live,
            "Other" => Self::Other,
            "Unknown" => Self::Unknown,
            other => Self::Custom(other.to_string()),
        }
    }
}

impl Type<Sqlite> for ReleaseGroupType {
    fn type_info() -> SqliteTypeInfo {
        <&str as Type<Sqlite>>::type_info()
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        <&str as Type<Sqlite>>::compatible(ty)
    }
}

impl<'q> Encode<'q, Sqlite> for ReleaseGroupType {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let owned = self.as_db_str().into_owned();
        <String as Encode<Sqlite>>::encode_by_ref(&owned, buf)
    }
}

impl<'r> Decode<'r, Sqlite> for ReleaseGroupType {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Sqlite>>::decode(value)?;
        Ok(Self::from_db_str(s))
    }
}

impl IntoUpdateValue for ReleaseGroupType {
    fn into_update_value(&self) -> UpdateValue {
        UpdateValue::Text(self.as_db_str().into_owned())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(release_groups)]
pub struct ReleaseGroup {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub primary_type: ReleaseGroupType,
}

impl ReleaseGroup {
    pub fn new(id: Uuid, artist_id: Uuid, title: String, primary_type: ReleaseGroupType) -> Self {
        Self {
            id,
            artist_id,
            title,
            primary_type,
        }
    }
}
