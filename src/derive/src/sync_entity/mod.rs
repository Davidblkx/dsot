mod generator;
mod ir;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

pub fn expand(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ir::parse_sync_entity(&ast) {
        Ok(output) => output.generate().into(),
        Err(e) => e.into_compile_error().into(),
    }
}
