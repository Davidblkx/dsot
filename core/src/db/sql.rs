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

/// Sanitizes sql parameters by wrapping the parameter in single quotes and escaping any single quotes in the parameter.
pub fn sanitize_param(param: &str) -> String {
    format!("'{}'", param.replace("'", "''"))
}

impl DbOperation {
    pub fn generate_sql(&self) -> Result<String> {
        match self {
            DbOperation::Delete { id, entity } => {
                let db_ent = DbEntity::from_id(*entity).ok_or(DsotError::UnknownDbEntity(*entity))?;
                let id = sanitize_param(&id.to_string());
                Ok(format!("DELETE FROM {} WHERE id = {}", db_ent.table_name(), id))
            },
            DbOperation::Create { id: _, entity, data } => {
                let db_ent = DbEntity::from_id(*entity).ok_or(DsotError::UnknownDbEntity(*entity))?;
                let values = db_ent.get_values(data)?.iter().map(|v| sanitize_param(v)).collect::<Vec<String>>().join(", ");
                Ok(format!("INSERT INTO {} ({}) VALUES ({})", db_ent.table_name(), db_ent.columns().join(", "), values))
            },
            DbOperation::Update { id, entity, props } => {
                let db_ent = DbEntity::from_id(*entity).ok_or(DsotError::UnknownDbEntity(*entity))?;
                let set = props.iter().map(|(k, v)| format!("{} = {}", k, sanitize_param(v))).collect::<Vec<String>>().join(", ");
                let id = sanitize_param(&id.to_string());
                Ok(format!("UPDATE {} SET {} WHERE id = {}", db_ent.table_name(), set, id))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::BinModel;
    use crate::db::entities::artist::ArtistV0;

    #[test]
    fn test_generate_insert_sql() {
        let artist = ArtistV0 {
            id: uuid::Uuid::now_v7(),
            name: "Test Artist".to_string(),
        };
        let create_op = DbOperation::Create {
            id: artist.id,
            entity: DbEntity::Artist.to_id(),
            data: artist.serialize().unwrap(),
        };

        let expected = format!("INSERT INTO artists (id, name) VALUES ('{}', 'Test Artist')", artist.id);
        assert_eq!(create_op.generate_sql().unwrap(), expected);
    }

    #[test]
    fn test_generate_update_sql() {
        let id = uuid::Uuid::now_v7();
        let update_op = DbOperation::Update {
            id,
            entity: DbEntity::Artist.to_id(),
            props: vec![("name".to_string(), "Updated Artist".to_string())],
        };

        let expected = format!("UPDATE artists SET name = 'Updated Artist' WHERE id = '{}'", id);
        assert_eq!(update_op.generate_sql().unwrap(), expected);
    }

    #[test]
    fn test_generate_delete_sql() {
        let id = uuid::Uuid::now_v7();
        let delete_op = DbOperation::Delete {
            id,
            entity: DbEntity::Artist.to_id(),
        };

        let expected = format!("DELETE FROM artists WHERE id = '{}'", id);
        assert_eq!(delete_op.generate_sql().unwrap(), expected);
    }
}
