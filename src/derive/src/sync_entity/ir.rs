use syn::{Data, DeriveInput, Field, Fields, punctuated::Punctuated, token::Comma};

pub struct SyncEntityIR {
    pub name: syn::Ident,
    pub table_name: TableName,
    pub id_field: IdField,
    pub fields: Punctuated<Field, Comma>,
}

// TODO!: start writing generator and improve IR for this
pub fn parse_sync_entity(ast: &DeriveInput) -> syn::Result<SyncEntityIR> {
    Ok(SyncEntityIR {
        name: ast.ident.clone(),
        table_name: ast.into(),
        id_field: ast.try_into()?,
        fields: collect_fields(ast)?.clone(),
    })
}

pub struct TableName(String);

impl From<&DeriveInput> for TableName {
    fn from(ast: &DeriveInput) -> Self {
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

        Self(name)
    }
}

pub struct IdField(syn::Field);

impl TryFrom<&DeriveInput> for IdField {
    type Error = syn::Error;

    fn try_from(ast: &DeriveInput) -> Result<Self, Self::Error> {
        let mut id_field = None;
        let mut found_id_attr = false;

        for f in collect_fields(ast)? {
            for attr in &f.attrs {
                if attr.path().is_ident("id") {
                    found_id_attr = true;
                    id_field = Some(f.clone());
                    break;
                }
            }

            if found_id_attr {
                break;
            }

            if f.ident.as_ref().is_some_and(|i| i == "id") {
                id_field = Some(f.clone())
            }
        }

        match id_field {
            Some(id) => Ok(Self(id)),
            None => Err(syn::Error::new_spanned(
                &ast.ident,
                "SyncEntity must have an id field or use #[id] attribute",
            )),
        }
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
