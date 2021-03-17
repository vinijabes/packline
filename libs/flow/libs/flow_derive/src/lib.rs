mod schema;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FlowSchema)]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    schema::expand_derive_schema(input).unwrap().into()
}
