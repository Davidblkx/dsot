use quote::quote;

use super::ir::*;

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
        } = field_data;

        let sql_name = quote::format_ident!("{}Sql", &name);
        let mut sql_fields: Vec<_> = fields.iter().map(|f| quote! { #f }).collect();

        if !has_deleted {
            sql_fields.push(quote! { pub is_deleted: bool });
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
            from_src_to_sql.push(quote! { is_deleted: false });
        }

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
        }
    }
}
