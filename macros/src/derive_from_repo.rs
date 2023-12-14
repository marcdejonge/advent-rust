extern crate proc_macro;
extern crate syn;

use proc_macro2::TokenStream;
use syn::{Attribute, Meta};

pub fn generate_from(ast: syn::DeriveInput) -> Result<TokenStream, String> {
    let name = &ast.ident;

    let rep = ast
        .attrs
        .iter()
        .filter_map(|attr| {
            if let Meta::List(path) = &attr.meta {
                if path.path.segments.first().unwrap().ident == "repr" {
                    if let Some(ident) = path.tokens.clone().into_iter().next() {
                        return Some(ident);
                    }
                }
            }
            None
        })
        .next()
        .ok_or(format!("#[repr(_)] wasn't found for {}", name))?;

    if let syn::Data::Enum(ref d) = ast.data {
        let variants: Vec<_> = d
            .variants
            .iter()
            .map(|v| {
                let ident = &v.ident;
                let (_, lit) = v.discriminant.clone().unwrap_or_else(|| {
                    panic!("#[derive(FromRepr)] No variant for {}::{}", name, ident)
                });
                let is_default = v.attrs.iter().any(|attr| attr_name_is(attr, "default"));

                (ident, lit, is_default)
            })
            .collect();

        if variants.is_empty() {
            return Err("#[derive(FromRepr)] Found multiple default values".to_string());
        }

        if variants.iter().filter(|(_, _, is_default)| *is_default).count() > 1 {
            return Err("#[derive(FromRepr)] Found multiple default values".to_string());
        }

        let default_variant = variants
            .iter()
            .find(|(_, _, is_default)| *is_default)
            .or(variants.first())
            .map(|(ident, _, _)| *ident)
            .unwrap();

        let from_literal_mappings: Vec<_> = variants
            .iter()
            .map(|(ident, lit, _)| {
                quote! { #lit => #name::#ident, }
            })
            .collect();

        let from_nr_mappings: Vec<_> = variants
            .iter()
            .map(|(ident, lit, _)| {
                quote! { #name::#ident => #lit, }
            })
            .collect();

        Ok(quote! {
            impl From<#rep> for #name {
                fn from(value: #rep) -> Self {
                    match value {
                        #(#from_literal_mappings)*
                        _ => #name::#default_variant,
                    }
                }
            }

            impl From<#name> for #rep {
                fn from(value: #name) -> Self {
                    match value {
                        #(#from_nr_mappings)*
                    }
                }
            }
        })
    } else {
        panic!("#[derive(FromRepr)] is only defined for Enum")
    }
}

fn attr_name_is(attr: &Attribute, name: &str) -> bool {
    if let Meta::List(path) = &attr.meta {
        if path.path.segments.first().unwrap().ident == name {
            return true;
        }
    }
    false
}
