use syn::{Data, DeriveInput, Field, Fields, punctuated::Punctuated, token::Comma};

use super::ir::*;

static ID_FIELD_NAME: &'static str = "id";
pub static DELETED_FIELD_NAME: &'static str = "deleted";
pub static CREATED_FIELD_NAME: &'static str = "created";
pub static UPDATED_FIELD_NAME: &'static str = "updated";

impl SyncEntityIR {
    pub fn parse(ast: &DeriveInput) -> syn::Result<Self> {
        Ok(SyncEntityIR {
            name: ast.ident.clone(),
            table_name: get_table_name(ast),
            field_data: ast.try_into()?,
        })
    }
}

fn get_table_name(ast: &DeriveInput) -> String {
    let mut name = ast.ident.to_string().to_lowercase();

    for att in &ast.attrs {
        if att.path().is_ident("table") {
            _ = att.parse_nested_meta(|m| {
                if let Some(ident) = m.path.get_ident() {
                    name = ident.to_string();
                }
                Ok(())
            });
        }
    }

    name
}

impl TryFrom<&DeriveInput> for SyncEntityFields {
    type Error = syn::Error;

    fn try_from(ast: &DeriveInput) -> Result<Self, Self::Error> {
        let mut id_field = None;
        let mut id_attr_found = false;
        let mut has_deleted = false;
        let mut has_created = false;
        let mut has_updated = false;

        let fields = collect_fields(&ast)?;

        for f in collect_fields(ast)? {
            for attr in &f.attrs {
                if attr.path().is_ident(ID_FIELD_NAME) {
                    if id_attr_found {
                        return Err(syn::Error::new_spanned(attr, "Multiple id fields found"));
                    }

                    id_attr_found = true;
                    id_field = Some(f.clone());
                }
            }

            if id_field.is_none() && f.ident.as_ref().is_some_and(|i| i == ID_FIELD_NAME) {
                id_field = Some(f.clone())
            }

            if f.ident.as_ref().is_some_and(|i| i == DELETED_FIELD_NAME) {
                has_deleted = true;
            }

            if f.ident.as_ref().is_some_and(|i| i == CREATED_FIELD_NAME) {
                has_created = true;
            }

            if f.ident.as_ref().is_some_and(|i| i == UPDATED_FIELD_NAME) {
                has_updated = true;
            }
        }

        let id_field = match id_field {
            Some(id) => Ok(id),
            None => Err(syn::Error::new_spanned(
                &ast.ident,
                "SyncEntity must have an id field or use #[id] attribute",
            )),
        }?;

        Ok(Self {
            id: id_field,
            fields: fields.clone(),
            has_deleted,
            has_created,
            has_updated,
        })
    }
}

fn collect_fields<'a>(ast: &'a DeriveInput) -> syn::Result<&'a Punctuated<Field, Comma>> {
    match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => Ok(&fields.named),
            _ => Err(syn::Error::new_spanned(
                &ast.ident,
                "SyncEntity macro only supports structs with named fields",
            )),
        },
        _ => Err(syn::Error::new_spanned(
            &ast.ident,
            "SyncEntity macro only supports structs",
        )),
    }
}
