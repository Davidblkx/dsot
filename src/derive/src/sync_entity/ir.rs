use syn::{Field, punctuated::Punctuated, token::Comma};

pub struct SyncEntityIR {
    pub name: syn::Ident,
    pub table_name: String,
    pub field_data: SyncEntityFields,
}

pub struct SyncEntityFields {
    pub id: Field,
    pub fields: Punctuated<Field, Comma>,
    pub has_deleted: bool,
    pub has_created: bool,
    pub has_updated: bool,
}
