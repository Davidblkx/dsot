use super::{Artist, op::ArtistUpdateOp};
use crate::model::entities::rel::ArtistAlias;

crate::dsot_sql_entity!(["artists"] Artist with ArtistUpdateOp {
    mbid,
    name,
    sort_name,
    artist_type_id
});

impl Artist {
    pub async fn get_aliases(&self, mut trx: SqlTransaction) -> SqlResult<Vec<ArtistAlias>> {
        let rows = sqlx::query_as::<_, ArtistAlias>(
            "SELECT * FROM artist_aliases WHERE artist_id = ?"
        )
        .bind(self.id)
        .fetch_all(&mut *trx)
        .await?;

        Ok((trx, rows))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::sql::ArtistSql;
    use crate::model::entities::rel::{ArtistAlias, ArtistAliasSql};

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query_aliases(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let artist = Artist::new("artist");

        let (mut trx, _) = ArtistSql::insert(trx, &artist).await.unwrap();

        let aliases = vec![
            String::from("alias1"),
            String::from("alias2"),
            String::from("alias3"),
        ];

        for alias in &aliases {
            (trx, _) = ArtistAliasSql::insert(trx, &ArtistAlias::new(&artist.id, alias)).await.unwrap();
        }

        let (_, fetched_aliases) = artist.get_aliases(trx).await.unwrap();
        let names = fetched_aliases.iter().map(|a| a.name.clone()).collect::<Vec<_>>();

        for alias in &aliases {
            assert!(names.contains(alias), "Alias {:?} not found in fetched aliases", alias);
        }

        for alias in &fetched_aliases {
            assert_eq!(alias.artist_id, artist.id, "Alias artist_id does not match");
        }
    }
}
