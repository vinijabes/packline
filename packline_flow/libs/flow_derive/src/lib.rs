mod schema;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FlowSerializable)]
pub fn derive_serializable_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    schema::expand_derive_serializable_schema(input)
        .unwrap()
        .into()
}

#[proc_macro_derive(FlowDeserializable)]
pub fn derive_deserializable_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    schema::expand_derive_deserializable_schema(input)
        .unwrap()
        .into()
}

#[proc_macro_derive(FlowSized)]
pub fn derive_sized_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    schema::expand_derive_sized_schema(input).unwrap().into()
}
