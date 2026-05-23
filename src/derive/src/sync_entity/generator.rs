use quote::quote;

use super::ir::*;
use super::parser::{CREATED_FIELD_NAME, DELETED_FIELD_NAME, UPDATED_FIELD_NAME};

impl SyncEntityIR {
    pub fn generate(self) -> proc_macro2::TokenStream {
        let SyncEntityIR {
            name,
            table_name,
            field_data,
        } = self;

        let sql_ident = quote::format_ident!("{}Sql", &name);
        let repo_ident = quote::format_ident!("{}SqlRepository", &name);

        /*
         * Generate the SQL entity struct and it's mapping (From/To) to base entity
         *
         * e.g.
         * Artist -> ArtistSql
         */
        let create_sql_entity = SyncEntityIR::generate_sql_entity(&name, &sql_ident, &field_data);
        /*
         * Generate the SyncEntity implementation for the SQL entity
         *
         * impl SyncEntity for ArtistSql { ... }
         */
        let impl_sync_entity = SyncEntityIR::generate_impl_sync_entity(&sql_ident, &field_data);
        /*
         * Generates repository for the SQL entity
         *
         * pub struct ArtistSqlRepository;
         * impl ArtistSqlRepository { ... }
         */
        let impl_repo =
            SyncEntityIR::generate_repository(&sql_ident, &repo_ident, &table_name, &field_data);

        let ref_ident = quote::format_ident!("{}_REPOSITORY_REF", &name.to_string().to_uppercase());

        quote! {
            #create_sql_entity

            #impl_sync_entity

            #impl_repo

            #[::linkme::distributed_slice(::dsot_db_sync::registry::APPLY_JOURNAL_REF)]
            static #ref_ident: ::dsot_db_sync::registry::ApplyJournalRef =
                dsot_db_sync::registry::ApplyJournalRef {
                    table: stringify!(#table_name),
                    apply_journal: |db, journal| {
                        Box::pin(async move { db.apply_journal::<#repo_ident>(journal).await })
                    },
                };
        }
    }

    fn generate_repository(
        sql_entity_ident: &syn::Ident,
        repo_ident: &syn::Ident,
        table: &str,
        fdata: &SyncEntityFields,
    ) -> proc_macro2::TokenStream {
        let mut insert_fields: Vec<_> = fdata
            .fields
            .iter()
            .filter(|e| e.ident.is_some())
            .map(|f| f.ident.clone().unwrap())
            .collect();
        if !fdata.has_created {
            insert_fields.push(quote::format_ident!("{}", CREATED_FIELD_NAME));
        }
        if !fdata.has_updated {
            insert_fields.push(quote::format_ident!("{}", UPDATED_FIELD_NAME));
        }
        if !fdata.has_deleted {
            insert_fields.push(quote::format_ident!("{}", DELETED_FIELD_NAME));
        }

        let placeholders: Vec<String> = vec!["?".to_string(); insert_fields.len()];
        let placeholders_str = placeholders.join(", ");
        let columns = insert_fields
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let insert_query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table, columns, placeholders_str
        );

        let id_str = fdata
            .id
            .ident
            .as_ref()
            .map(|f| f.to_string())
            .unwrap_or("id".to_string());

        let delete_query = format!(
            "UPDATE {} SET deleted = 1, updated = ? WHERE {:?} = ?",
            table, id_str
        );
        let restore_query = format!(
            "UPDATE {} SET deleted = 0, updated = ? WHERE {:?} = ?",
            table, id_str
        );

        let update_query_set = format!("UPDATE {} SET ", table);
        let update_query_where = format!(" WHERE {} = ", id_str);

        let mut select_expr: Vec<_> = fdata
            .fields
            .iter()
            .map(|f| {
                let col_name = f.ident.as_ref().unwrap().to_string();
                let f_type = &f.ty;
                let mut type_str = quote! { #f_type }.to_string().replace(" ", "");
                if type_str.starts_with("Option<") {
                    type_str = type_str[7..type_str.len() - 1].to_string();
                }

                format!(r#"{} AS "{}: {}""#, col_name, col_name, type_str)
            })
            .collect();
        if !fdata.has_created {
            select_expr
                .push(r#"created AS "created: ::chrono::DateTime<::chrono::Utc>""#.to_string());
        }
        if !fdata.has_updated {
            select_expr
                .push(r#"updated AS "updated: ::chrono::DateTime<::chrono::Utc>""#.to_string());
        }
        if !fdata.has_deleted {
            select_expr.push(r#"deleted AS "deleted: bool""#.to_string());
        }

        let select_query_cols = format!("SELECT {} FROM {}", select_expr.join(", "), table);
        let select_by_id = format!("{} WHERE {} = $1", select_query_cols, id_str);
        let select_by_query = format!(
            "{} ORDER BY {} ASC LIMIT ? OFFSET ?",
            select_query_cols, id_str
        );

        let mut select_expr_fts: Vec<_> = fdata
            .fields
            .iter()
            .map(|f| {
                let col_name = f.ident.as_ref().unwrap().to_string();
                let f_type = &f.ty;
                let mut type_str = quote! { #f_type }.to_string().replace(" ", "");
                if type_str.starts_with("Option<") {
                    type_str = type_str[7..type_str.len() - 1].to_string();
                }

                format!(r#"a.{} AS "{}: {}""#, col_name, col_name, type_str)
            })
            .collect();
        if !fdata.has_created {
            select_expr_fts
                .push(r#"a.created AS "created: ::chrono::DateTime<::chrono::Utc>""#.to_string());
        }
        if !fdata.has_updated {
            select_expr_fts
                .push(r#"a.updated AS "updated: ::chrono::DateTime<::chrono::Utc>""#.to_string());
        }
        if !fdata.has_deleted {
            select_expr_fts.push(r#"a.deleted AS "deleted: bool""#.to_string());
        }

        let search_query = format!(
            "SELECT {} FROM {} a JOIN {}_fts f ON a.{} = f.{} WHERE {}_fts MATCH ? AND a.deleted = 0 ORDER BY f.rank",
            select_expr_fts.join(", "),
            table, table, id_str, id_str, table
        );

        quote! {
            pub struct #repo_ident;
            impl ::dsot_db_sync::repo::SyncEntityRepository for #repo_ident {
                type RepoEntity = #sql_entity_ident;

                fn get_table_name() -> &'static str {
                    stringify!(#table)
                }

                async fn insert(executor: &mut ::sqlx::SqliteConnection, entity: &#sql_entity_ident) -> ::dsot_db_sync::repo::Result<()>
                {
                    ::sqlx::query!(
                        #insert_query,
                        #(entity.#insert_fields),*
                    )
                    .execute(executor)
                    .await?;

                    Ok(())
                }

                async fn delete(executor: &mut ::sqlx::SqliteConnection, id: ::uuid::Uuid) -> dsot_db_sync::repo::Result<()>
                {
                    let now = ::chrono::Utc::now();
                    sqlx::query!(
                        #delete_query,
                        now,
                        id,
                    )
                    .execute(executor)
                    .await?;

                    Ok(())
                }

                async fn restore(executor: &mut ::sqlx::SqliteConnection, id: ::uuid::Uuid) -> dsot_db_sync::repo::Result<()>
                {
                    let now = ::chrono::Utc::now();
                    sqlx::query!(
                        #restore_query,
                        now,
                        id,
                    )
                    .execute(executor)
                    .await?;

                    Ok(())
                }

                async fn update(
                    executor: &mut ::sqlx::SqliteConnection,
                    id: ::uuid::Uuid,
                    updates: Vec<dsot_db_sync::model::UpdateColumnOp>,
                ) -> dsot_db_sync::repo::Result<()>
                {
                    if updates.is_empty() {
                        return Ok(());
                    }

                    let mut query_builder = ::sqlx::QueryBuilder::<::sqlx::Sqlite>::new(#update_query_set);

                    for (i, op) in updates.iter().enumerate() {
                        if i > 0 {
                            query_builder.push(", ");
                        }
                        query_builder.push(format!("{} = ", op.column));
                        match &op.value {
                            dsot_db_sync::model::UpdateValue::Null => {
                                query_builder.push_bind(None::<String>);
                            }
                            dsot_db_sync::model::UpdateValue::Integer(v) => {
                                query_builder.push_bind(*v);
                            }
                            dsot_db_sync::model::UpdateValue::Real(v) => {
                                query_builder.push_bind(*v);
                            }
                            dsot_db_sync::model::UpdateValue::Text(v) => {
                                query_builder.push_bind(v);
                            }
                            dsot_db_sync::model::UpdateValue::Blob(v) => {
                                query_builder.push_bind(v);
                            }
                        }
                    }

                    query_builder.push(#update_query_where);
                    query_builder.push_bind(id);

                    let query = query_builder.build();
                    query.execute(executor).await?;

                    Ok(())
                }

                async fn get(executor: &mut ::sqlx::SqliteConnection, id: ::uuid::Uuid) -> ::dsot_db_sync::repo::Result<#sql_entity_ident>
                {
                    let value = ::sqlx::query_as!(
                        #sql_entity_ident,
                        #select_by_id,
                        id
                    )
                    .fetch_optional(executor)
                    .await?;

                    match value {
                        Some(v) => Ok(v),
                        None => Err(::dsot_db_sync::repo::RepositoryError::EntityNotFound(
                            stringify!(#sql_entity_ident),
                            id.clone(),
                        )),
                    }
                }

                async fn try_get(executor: &mut ::sqlx::SqliteConnection, id: ::uuid::Uuid) -> ::dsot_db_sync::repo::Result<Option<#sql_entity_ident>>
                {
                    let value = ::sqlx::query_as!(
                        #sql_entity_ident,
                        #select_by_id,
                        id
                    )
                    .fetch_optional(executor)
                    .await?;

                    Ok(value)
                }

                async fn list(
                    executor: &mut ::sqlx::SqliteConnection,
                    query: ::dsot_db_sync::repo::ListQuery,
                ) -> ::dsot_db_sync::repo::Result<Vec<#sql_entity_ident>>
                {
                    let ::dsot_db_sync::repo::ListQuery { count, offset } = query;
                    let value = ::sqlx::query_as!(
                        #sql_entity_ident,
                        #select_by_query,
                        count,
                        offset
                    )
                    .fetch_all(executor)
                    .await?;

                    Ok(value)
                }

                async fn search(
                    executor: &mut ::sqlx::SqliteConnection,
                    query: String,
                ) -> ::dsot_db_sync::repo::Result<Vec<#sql_entity_ident>>
                {
                    let value = ::sqlx::query_as!(
                        #sql_entity_ident,
                        #search_query,
                        query
                    )
                    .fetch_all(executor)
                    .await?;

                    Ok(value)
                }

                async fn exec_op(
                    executor: &mut ::sqlx::SqliteConnection,
                    op: ::dsot_db_sync::model::SyncOperation,
                ) -> ::dsot_db_sync::repo::Result<()>
                {
                    match op {
                        ::dsot_db_sync::model::SyncOperation::Create(data) => {
                            let value = <#sql_entity_ident as ::dsot_db_sync::SyncEntity>::from_bytes(&data)?;
                            Self::insert(executor, &value).await
                        }
                        ::dsot_db_sync::model::SyncOperation::Update(id, updates) => {
                            Self::update(executor, id, updates).await
                        }
                        ::dsot_db_sync::model::SyncOperation::Delete(id) => {
                            Self::delete(executor, id).await
                        }
                        ::dsot_db_sync::model::SyncOperation::Restore(id) => {
                            Self::restore(executor, id).await
                        }
                    }
                }
            }
        }
    }

    fn generate_impl_sync_entity(
        sql_entity_ident: &syn::Ident,
        fdata: &SyncEntityFields,
    ) -> proc_macro2::TokenStream {
        let id = &fdata.id;
        let id_ident = &id.ident;

        let update_fields: Vec<_> = fdata
            .fields
            .iter()
            .filter(|f| !f.ident.as_ref().is_some_and(|i| i == DELETED_FIELD_NAME))
            .filter(|f| !f.ident.as_ref().is_some_and(|i| i == CREATED_FIELD_NAME))
            .filter(|f| !f.ident.as_ref().is_some_and(|i| i == UPDATED_FIELD_NAME))
            .filter(|f| f.ident != id.ident)
            .map(|f| &f.ident)
            .collect();

        quote! {
            impl ::dsot_db_sync::SyncEntity for #sql_entity_ident {
                type Entity = #sql_entity_ident;

                fn get_id(&self) -> Uuid {
                    self.#id_ident
                }

                fn op_create(&self) -> ::dsot_db_sync::dser::Result<::dsot_db_sync::model::SyncOperation> {
                    let value = self.to_bytes()?;
                    Ok(::dsot_db_sync::model::SyncOperation::Create(value))
                }

                fn op_delete(&self) -> ::dsot_db_sync::model::SyncOperation {
                    ::dsot_db_sync::model::SyncOperation::Delete(self.id)
                }

                fn op_restore(&self) -> ::dsot_db_sync::model::SyncOperation {
                    ::dsot_db_sync::model::SyncOperation::Restore(self.id)
                }

                fn op_update(&self, prev: &Self::Entity) -> Option<::dsot_db_sync::model::SyncOperation> {
                    if self.#id_ident != prev.#id_ident {
                        return None;
                    }

                    let mut list: Vec<dsot_db_sync::model::UpdateColumnOp> = Vec::new();

                    #(
                        if let Some(value) = ::dsot_db_sync::model::UpdateValue::get_if_diff(&prev.#update_fields, &self.#update_fields) {
                            list.push(::dsot_db_sync::model::UpdateColumnOp {
                                column: stringify!(#update_fields).to_string(),
                                value,
                            });
                        }
                    )*

                    if list.len() > 0 {
                        list.push(::dsot_db_sync::model::UpdateColumnOp {
                            column: "updated".to_string(),
                            value: ::dsot_db_sync::model::UpdateValue::from_utc_now(),
                        });
                        Some(::dsot_db_sync::model::SyncOperation::Update(self.id, list))
                    } else {
                        None
                    }
                }

                fn to_bytes(&self) -> ::dsot_db_sync::dser::Result<Vec<u8>> {
                    ::dsot_db_sync::dser::EntityMessagePack::serialize(self)
                }

                fn from_bytes(data: &[u8]) -> ::dsot_db_sync::dser::Result<Self::Entity> {
                    ::dsot_db_sync::dser::EntityMessagePack::deserialize(data)
                }
            }
        }
    }

    fn generate_sql_entity(
        entity_ident: &syn::Ident,
        sql_entity_ident: &syn::Ident,
        fdata: &SyncEntityFields,
    ) -> proc_macro2::TokenStream {
        let mut sql_entity_fields: Vec<_> = fdata.fields.iter().map(|f| quote! { #f }).collect();

        if !fdata.has_deleted {
            sql_entity_fields.push(quote! { pub deleted: bool });
        }

        if !fdata.has_created {
            sql_entity_fields.push(quote! { pub created: ::chrono::DateTime<::chrono::Utc> });
        }

        if !fdata.has_updated {
            sql_entity_fields.push(quote! { pub updated: ::chrono::DateTime<::chrono::Utc> });
        }
        let from_sql_to_src: Vec<_> = fdata
            .fields
            .iter()
            .map(|f| {
                let ident = &f.ident;
                quote! { #ident: value.#ident }
            })
            .collect();
        let mut from_src_to_sql = from_sql_to_src.clone();
        if !fdata.has_deleted {
            from_src_to_sql.push(quote! { deleted: false });
        }
        if !fdata.has_updated {
            from_src_to_sql.push(quote! { updated: ::chrono::Utc::now() });
        }
        if !fdata.has_created {
            from_src_to_sql.push(quote! { created: ::chrono::Utc::now() });
        }

        quote::quote! {
            #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, Default, ::sqlx::FromRow)]
            pub struct #sql_entity_ident {
                #(#sql_entity_fields),*
            }

            impl From<#entity_ident> for #sql_entity_ident {
                fn from(value: #entity_ident) -> Self {
                    Self {
                        #(#from_src_to_sql),*
                    }
                }
            }

            impl From<#sql_entity_ident> for #entity_ident {
                fn from(value: #sql_entity_ident) -> Self {
                    Self {
                        #(#from_sql_to_src),*
                    }
                }
            }

            impl ::dsot_db_sync::IntoSyncEntity for #entity_ident {
                type Entity = #sql_entity_ident;

                fn to_sync(&self) -> Self::Entity {
                    self.clone().into()
                }
            }
        }
    }
}
