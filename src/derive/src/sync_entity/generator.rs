use quote::quote;

use super::ir::SyncEntityIR;

impl SyncEntityIR {
    pub fn generate(&self) -> proc_macro2::TokenStream {
        quote! {}
    }
}
