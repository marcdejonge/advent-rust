mod derive_from_repo;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(FromRepr)]
pub fn from_repr_enum(input: TokenStream) -> TokenStream {
    derive_from_repo::generate_from(syn::parse_macro_input!(input as DeriveInput))
        .unwrap()
        .into()
}
