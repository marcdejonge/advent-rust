#[macro_use]
extern crate quote;
mod derive_from_repr;

use proc_macro::TokenStream;
use syn::{DeriveInput, Error};

#[proc_macro_derive(FromRepr, attributes(display, default))]
pub fn from_repr_enum(input: TokenStream) -> TokenStream {
    derive_from_repr::generate_from(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
