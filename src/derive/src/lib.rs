mod sync_entity;

use proc_macro::TokenStream;

#[proc_macro_derive(SyncEntity, attributes(id, table))]
pub fn sync_entity_derive(input: TokenStream) -> TokenStream {
    sync_entity::expand(input)
}
