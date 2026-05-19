use syn::{Field, punctuated::Punctuated, token::Comma};

pub struct SyncEntityIR {
    pub name: syn::Ident,
    pub table_name: TableName,
    pub field_data: SyncEntityFields,
}

pub struct TableName(pub String);

pub struct SyncEntityFields {
    pub id: Field,
    pub fields: Punctuated<Field, Comma>,
    pub has_deleted: bool,
}
