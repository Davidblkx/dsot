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

        let SyncEntityFields {
            id,
            fields,
            has_deleted,
            has_created,
            has_updated,
        } = field_data;

        let sql_name = quote::format_ident!("{}Sql", &name);
        let id_ident = &id.ident.clone();
        let mut sql_fields: Vec<_> = fields.iter().map(|f| quote! { #f }).collect();

        if !has_deleted {
            sql_fields.push(quote! { pub deleted: bool });
        }

        if !has_created {
            sql_fields.push(quote! { pub created: ::chrono::DateTime<::chrono::Utc> });
        }

        if !has_updated {
            sql_fields.push(quote! { pub updated: ::chrono::DateTime<::chrono::Utc> });
        }

        let from_sql_to_src: Vec<_> = fields
            .iter()
            .map(|f| {
                let ident = &f.ident;
                quote! { #ident: value.#ident }
            })
            .collect();
        let mut from_src_to_sql = from_sql_to_src.clone();
        if !has_deleted {
            from_src_to_sql.push(quote! { deleted: false });
        }
        if !has_updated {
            from_src_to_sql.push(quote! { updated: ::chrono::Utc::now() });
        }
        if !has_created {
            from_src_to_sql.push(quote! { created: ::chrono::Utc::now() });
        }

        let update_fields: Vec<_> = fields
            .iter()
            .filter(|f| !f.ident.as_ref().is_some_and(|i| i == DELETED_FIELD_NAME))
            .filter(|f| !f.ident.as_ref().is_some_and(|i| i == CREATED_FIELD_NAME))
            .filter(|f| !f.ident.as_ref().is_some_and(|i| i == UPDATED_FIELD_NAME))
            .filter(|f| f.ident != id.ident)
            .map(|f| &f.ident)
            .collect();

        quote! {
            #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, Default, ::sqlx::FromRow)]
            pub struct #sql_name {
                #(#sql_fields),*
            }

            impl From<#name> for #sql_name {
                fn from(value: #name) -> Self {
                    Self {
                        #(#from_src_to_sql),*
                    }
                }
            }

            impl From<#sql_name> for #name {
                fn from(value: #sql_name) -> Self {
                    Self {
                        #(#from_sql_to_src),*
                    }
                }
            }

            impl #name {
                pub fn to_sql(self) -> #sql_name {
                    self.into()
                }
            }

            impl ::dsot_db_sync::SyncEntity for #sql_name {
                type Entity = #sql_name;

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
}
