use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{self, parse_quote, spanned::Spanned, Data, Fields, GenericParam, Generics};

pub fn expand_derive_schema(input: syn::DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let name = &input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expand_deserialize = deserialize(name, &input.data);
    let expand_size = size_of(&input.data);

    let expanded = quote! {
        // ...
        impl #impl_generics flow::Schema for #name #ty_generics #where_clause{
            fn deserialize() -> Self {
                #expand_deserialize
            }

            fn serialize() {
            }

            fn size(&self) -> usize{
                #expand_size
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(flow::Schema));
        }
    }
    generics
}

fn deserialize(struct_name: &proc_macro2::Ident, data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse_deserialize = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;

                    quote_spanned! {f.span()=>
                        let #name = #ty::deserialize();
                    }
                });

                let recurse_fields = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {f.span()=>
                        #name
                    }
                });

                quote! {
                    #(
                        #recurse_deserialize
                    )*

                    #struct_name {
                        #(#recurse_fields),*
                    }
                }
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn size_of(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! (f.span()=>flow::Schema::size(&self.#name))
                });

                quote! {
                    0 #(+ #recurse)*
                }
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
