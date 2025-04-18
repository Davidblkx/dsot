use uuid::Uuid;

use music_brainz::model::artist::ArtistType;

use crate::storage::{sql::SqlTable, BinModel, SqlOperationHandler};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ArtistV0 {
    pub id: Uuid,
    pub mbid: Option<Uuid>,
    pub name: String,
    pub sort_name: Option<String>,
    pub artist_type_id: u32,
}

crate::dsot_storage_declare_model!(Artist {
    0: ArtistV0
});

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ArtistUpdateOpV0 {
    SetMbid(Uuid),
    SetName(String),
    SetSortName(String),
    SetArtistTypeId(u32),
}

crate::dsot_storage_declare_model!(ArtistUpdateOp {
    0: ArtistUpdateOpV0
});

impl Artist {
    pub fn get_artist_type(&self) -> ArtistType {
        ArtistType::from_u32(self.artist_type_id)
    }

    pub fn set_artist_type(&mut self, artist_type: ArtistType) {
        self.artist_type_id = artist_type.to_u32();
    }
}

impl Default for Artist {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            mbid: None,
            name: String::new(),
            sort_name: None,
            artist_type_id: 0,
        }
    }
}

crate::dsot_sql_entity!(["artists"] Artist with ArtistUpdateOp {
    mbid,
    name,
    sort_name,
    artist_type_id
});

impl SqlTable for Artist {
    fn table() -> &'static str {
        "artists"
    }

    type Entity = Artist;
    type UpdateOp = ArtistUpdateOp;

    async fn execute_create(
        mut trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        entity: &Self::Entity
    ) -> crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>> {
        sqlx::query!(
            r#"
INSERT INTO artists (id, mbid, name, sort_name, artist_type_id)
VALUES (?, ?, ?, ?, ?)
            "#,
            entity.id,
            entity.mbid,
            entity.name,
            entity.sort_name,
            entity.artist_type_id
        )
        .execute(&mut *trx)
        .await?;

        Ok(trx)
    }

    async fn execute_update(
        mut trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        id: &uuid::Uuid,
        op: &Self::UpdateOp
    ) -> crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>> {
        match op {
            ArtistUpdateOp::SetMbid(mbid) => {
                sqlx::query!(
                    r#"
                        UPDATE artists
                        SET mbid = ?
                        WHERE id = ?
                    "#,
                    mbid,
                    id
                )
                .execute(&mut *trx)
                .await?;
            },
            ArtistUpdateOp::SetName(name) => {
                sqlx::query!(
                    r#"
                        UPDATE artists
                        SET name = ?
                        WHERE id = ?
                    "#,
                    name,
                    id
                )
                .execute(&mut *trx)
                .await?;
            },
            ArtistUpdateOp::SetSortName(sort_name) => {
                sqlx::query!(
                    r#"
                        UPDATE artists
                        SET sort_name = ?
                        WHERE id = ?
                    "#,
                    sort_name,
                    id
                )
                .execute(&mut *trx)
                .await?;
            },
            ArtistUpdateOp::SetArtistTypeId(artist_type_id) => {
                sqlx::query!(
                    r#"
                        UPDATE artists
                        SET artist_type_id = ?
                        WHERE id = ?
                    "#,
                    artist_type_id,
                    id
                )
                .execute(&mut *trx)
                .await?;
            },
        }

        Ok(trx)
    }

    async fn execute_delete(
        mut trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        id: &uuid::Uuid
    ) -> crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>> {
        sqlx::query!(
            r#"
                DELETE FROM artists
                WHERE id = ?
            "#,
            id
        )
        .execute(&mut *trx)
        .await?;

        Ok(trx)
    }

    async fn execute_fetch(
        mut trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        id: &uuid::Uuid
    ) -> crate::error::Result<(sqlx::Transaction<'static, sqlx::Sqlite>, Self::Entity)> {
        let entity = sqlx::query_as::<_, Artist>(
            r#"
                SELECT id, mbid, name, sort_name, artist_type_id
                FROM artists
                WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_one(&mut *trx)
        .await?;

        Ok((trx, entity))
    }
}

impl SqlOperationHandler for Artist {
    async fn apply_sql_op(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        op: &crate::storage::SqlOperation,
    ) -> crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>> {
        match op {
            crate::storage::SqlOperation::Create { data, .. } => {
                let entity = Artist::deserialize(data)?;
                Self::execute_create(trx, &entity).await
            },
            crate::storage::SqlOperation::Update { id, action, .. } => {
                let op = ArtistUpdateOp::deserialize(action)?;
                Self::execute_update(trx, id, &op).await
            },
            crate::storage::SqlOperation::Delete { id, .. } => {
                Self::execute_delete(trx, id).await
            },
        }
    }
}
