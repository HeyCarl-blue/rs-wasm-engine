use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn component (_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        #input

        impl engine_wasm_rs_core::ecs::component::Component for #name {}
    };

    TokenStream::from(expanded)
}