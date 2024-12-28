#[macro_use]
extern crate quote;
mod derive_from_repr;
mod parser;

use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{DeriveInput, Error, Item};

#[proc_macro_derive(FromRepr, attributes(display, default))]
pub fn from_repr_enum(input: TokenStream) -> TokenStream {
    derive_from_repr::generate_from(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn format(_attr: TokenStream, input: TokenStream) -> TokenStream { input }

#[proc_macro_attribute]
pub fn parsable(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as Item);
    (match &item {
        Item::Struct(item) => {
            if attr.is_empty() {
                parser::generate_struct_parser(None, item)
            } else {
                let expr = syn::parse_macro_input!(attr as syn::Expr);
                parser::generate_struct_parser(Some(expr), item)
            }
        }
        Item::Enum(item) => parser::generate_enum_parser(item),
        _ => Err(Error::new(
            item.span(),
            "Only structs and enums are supported",
        )),
    })
    .unwrap_or_else(Error::into_compile_error)
    .into()
}
