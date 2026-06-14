use std::borrow::Cow;

use dsot_db_sync::{
    DBSyncError,
    dser::EntityMessagePack,
    model::{IntoUpdateValue, UpdateValue},
};
use dsot_derive::SyncEntity;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, Sqlite, Type, sqlite::SqliteTypeInfo};
use uuid::Uuid;

/// Typed payload of an inbox item. Stored as msgpack-encoded bytes in
/// `InboxItem::value`, so adding a new variant is a code-only change —
/// no migration required.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum InboxValue {
    /// A local file path or URI the user wants to import.
    File(String),
    /// An artist name to match against metadata provider.
    Artist(String),
    /// An album name to match against metadata provider.
    Album {
        album: String,
        artist: String,
        year: Option<u32>,
    },
    /// A link to a resource
    Link(String),
    /// Free-form catch-all.
    Other(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum InboxStatus {
    #[default]
    Pending,
    Resolved,
    Failed,
}

impl InboxStatus {
    pub fn as_db_str(&self) -> Cow<'_, str> {
        match self {
            Self::Pending => Cow::Borrowed("Pending"),
            Self::Resolved => Cow::Borrowed("Resolved"),
            Self::Failed => Cow::Borrowed("Failed"),
        }
    }

    pub fn from_db_str(s: &str) -> Result<Self, UnknownInboxStatus> {
        match s {
            "Pending" => Ok(Self::Pending),
            "Resolved" => Ok(Self::Resolved),
            "Failed" => Ok(Self::Failed),
            other => Err(UnknownInboxStatus(other.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("unknown inbox status: {0}")]
pub struct UnknownInboxStatus(pub String);

impl Type<Sqlite> for InboxStatus {
    fn type_info() -> SqliteTypeInfo {
        <&str as Type<Sqlite>>::type_info()
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        <&str as Type<Sqlite>>::compatible(ty)
    }
}

impl<'q> Encode<'q, Sqlite> for InboxStatus {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let owned = self.as_db_str().into_owned();
        <String as Encode<Sqlite>>::encode_by_ref(&owned, buf)
    }
}

impl<'r> Decode<'r, Sqlite> for InboxStatus {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Sqlite>>::decode(value)?;
        Ok(Self::from_db_str(s)?)
    }
}

impl IntoUpdateValue for InboxStatus {
    fn into_update_value(&self) -> UpdateValue {
        UpdateValue::Text(self.as_db_str().into_owned())
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, SyncEntity, PartialEq)]
#[table(inbox_items)]
pub struct InboxItem {
    pub id: Uuid,
    /// msgpack-encoded [`InboxValue`]. Decode via [`InboxItem::value`].
    pub value: Vec<u8>,
    pub status: InboxStatus,
    /// Set when `status == InboxStatus::Resolved`. The caller infers the
    /// target table from the decoded `InboxValue` kind.
    pub resolved_id: Option<Uuid>,
}

impl InboxItem {
    /// Capture a new pending item. Encodes `value` via msgpack.
    pub fn new(value: InboxValue) -> Result<Self, DBSyncError> {
        Ok(Self {
            id: Uuid::now_v7(),
            value: EntityMessagePack::serialize(&value)?,
            status: InboxStatus::default(),
            resolved_id: None,
        })
    }

    /// Decode the payload into its typed form.
    pub fn value(&self) -> Result<InboxValue, DBSyncError> {
        EntityMessagePack::deserialize(&self.value)
    }

    /// Mark this item as resolved against an existing domain entity.
    pub fn resolve(&mut self, target_id: Uuid) {
        self.status = InboxStatus::Resolved;
        self.resolved_id = Some(target_id);
    }

    /// Mark this item as having failed to match. Kept around for retry.
    pub fn mark_failed(&mut self) {
        self.status = InboxStatus::Failed;
        self.resolved_id = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_roundtrips_through_msgpack() {
        for value in [
            InboxValue::File("/music/foo.flac".into()),
            InboxValue::Artist("Pink Floyd".into()),
            InboxValue::Other("ask Bob about that bootleg".into()),
        ] {
            let item = InboxItem::new(value.clone()).unwrap();
            assert_eq!(item.value().unwrap(), value);
            assert_eq!(item.status, InboxStatus::Pending);
            assert!(item.resolved_id.is_none());
        }
    }

    #[test]
    fn resolve_sets_status_and_target() {
        let mut item = InboxItem::new(InboxValue::Artist("Tom Waits".into())).unwrap();
        let target = Uuid::now_v7();

        item.resolve(target);

        assert_eq!(item.status, InboxStatus::Resolved);
        assert_eq!(item.resolved_id, Some(target));
    }

    #[test]
    fn mark_failed_clears_resolved_id() {
        let mut item = InboxItem::new(InboxValue::File("/x".into())).unwrap();
        item.resolve(Uuid::now_v7());

        item.mark_failed();

        assert_eq!(item.status, InboxStatus::Failed);
        assert!(item.resolved_id.is_none());
    }

    #[test]
    fn inbox_status_db_str_roundtrips() {
        for s in [
            InboxStatus::Pending,
            InboxStatus::Resolved,
            InboxStatus::Failed,
        ] {
            let written = s.as_db_str().into_owned();
            assert_eq!(InboxStatus::from_db_str(&written).unwrap(), s);
        }
    }

    #[test]
    fn inbox_status_from_unknown_string_errors() {
        assert!(InboxStatus::from_db_str("Bogus").is_err());
    }
}
