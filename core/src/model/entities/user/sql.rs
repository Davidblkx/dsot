use super::{User, op::UserUpdateOp};

crate::dsot_sql_entity!(["users"] User with UserUpdateOp {
    name
});

impl UserSql {
    pub async fn fetch_by_user_name(
        mut trx: SqlTransaction,
        name: &str,
    ) -> SqlResult<Option<User>> {
        match sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = ?")
            .bind(name)
            .fetch_one(&mut *trx)
            .await
        {
            Ok(user) => Ok((trx, Some(user))),
            _ => Ok((trx, None)),
        }
    }

    pub async fn fetch_all(mut trx: SqlTransaction) -> SqlResult<Vec<User>> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&mut *trx)
            .await?;

        Ok((trx, users))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let user = User::new("my_user");
        let (trx, _) = UserSql::insert(trx, &user).await.unwrap();
        let (_, fetch_user) = UserSql::fetch_by_id(trx, &user.id).await.unwrap();

        let u = fetch_user.unwrap();

        assert_eq!(user.id, u.id);
        assert_eq!(user.name, u.name);
    }
}
