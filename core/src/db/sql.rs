use uuid::Uuid;

use super::op::DbOperation;
use super::entity::DbEntity;
use crate::error::{DsotError, Result};

/// Trait for SQL entities, allows to define the table name, columns and values for a SQL entity.
pub trait SqlEntity {
    /// Returns the name of the table.
    fn table_name() -> &'static str;
    /// Returns the columns of the table.
    fn columns() -> Vec<&'static str>;
    /// Returns the values of the table.
    fn values(&self) -> Vec<String>;
}

pub struct SqlValue;

impl SqlValue {
    pub fn string(value: &str) -> String {
        format!("'{}'", value.replace("'", "''"))
    }

    pub fn uuid(value: &Uuid) -> String {
        format!("X'{}'", value.to_string().replace("-", ""))
    }

    pub fn null() -> String {
        "NULL".to_string()
    }
}

impl DbOperation {
    pub fn generate_sql(&self) -> Result<String> {
        match self {
            DbOperation::Delete { id, entity } => {
                let db_ent: DbEntity = DbEntity::from_id(*entity).ok_or(DsotError::UnknownDbEntity(*entity))?;
                let id = SqlValue::uuid(id);
                Ok(format!("DELETE FROM {} WHERE id = {}", db_ent.table_name(), id))
            },
            DbOperation::Create { id: _, entity, data } => {
                let db_ent = DbEntity::from_id(*entity).ok_or(DsotError::UnknownDbEntity(*entity))?;
                let values = db_ent.get_values(data)?.join(", ");
                Ok(format!("INSERT INTO {} ({}) VALUES ({})", db_ent.table_name(), db_ent.columns().join(", "), values))
            },
            DbOperation::Update { id, entity, props } => {
                let db_ent = DbEntity::from_id(*entity).ok_or(DsotError::UnknownDbEntity(*entity))?;
                let set = props.iter().map(|(k, v)| format!("{} = {}", k, v)).collect::<Vec<String>>().join(", ");
                let id = SqlValue::uuid(id);
                Ok(format!("UPDATE {} SET {} WHERE id = {}", db_ent.table_name(), set, id))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::BinModel;
    use crate::db::entities::artist::Artist;

    #[test]
    fn sql_value_string() {
        assert_eq!(SqlValue::string("Test"), "'Test'");
        assert_eq!(SqlValue::string("Test'"), "'Test'''");
    }

    #[test]
    fn sql_value_uuid() {
        let uuid = uuid::Uuid::now_v7();
        assert_eq!(SqlValue::uuid(&uuid), format!("X'{}'", uuid.to_string().replace("-", "")));
    }

    #[test]
    fn test_generate_insert_sql() {
        let mut artist = Artist::new(Uuid::now_v7(), "Test' Artist");
        artist.artist_type = 1;

        let create_op = DbOperation::Create {
            id: artist.id,
            entity: DbEntity::Artist.to_id(),
            data: artist.serialize().unwrap(),
        };

        let id = SqlValue::uuid(&artist.id);
        let expected = format!("INSERT INTO artists (id, name, sort_name, artist_type) VALUES ({}, 'Test'' Artist', NULL, 1)", id);
        assert_eq!(create_op.generate_sql().unwrap(), expected);
    }

    #[test]
    fn test_generate_update_sql() {
        let id = uuid::Uuid::now_v7();
        let update_op = DbOperation::Update {
            id,
            entity: DbEntity::Artist.to_id(),
            props: vec![("name".to_string(), SqlValue::string("Updated Artist"))],
        };

        let art_id = SqlValue::uuid(&id);
        let expected = format!("UPDATE artists SET name = 'Updated Artist' WHERE id = {}", art_id);
        assert_eq!(update_op.generate_sql().unwrap(), expected);
    }

    #[test]
    fn test_generate_delete_sql() {
        let id = uuid::Uuid::now_v7();
        let delete_op = DbOperation::Delete {
            id,
            entity: DbEntity::Artist.to_id(),
        };

        let art_id = SqlValue::uuid(&id);
        let expected = format!("DELETE FROM artists WHERE id = {}", art_id);
        assert_eq!(delete_op.generate_sql().unwrap(), expected);
    }
}
