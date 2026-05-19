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

        quote! {
            #[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, Default, ::sqlx::FromRow)]
            pub struct #sql_name {
                #(#sql_fields),*
            }
        }
    }
}
