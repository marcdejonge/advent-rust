extern crate proc_macro;
extern crate syn;

use proc_macro2::{Ident, TokenStream};
use syn::spanned::Spanned;
use syn::{Attribute, Error, Expr, Lit, Meta};

pub fn generate_from(ast: syn::DeriveInput) -> Result<TokenStream, Error> {
    let name = &ast.ident;

    let rep = parse_ident_from(
        find_named_attr(&ast.attrs, "repr")
            .ok_or(Error::new(ast.ident.span(), "Missing #[repr(..)]"))?,
    )?;

    if let syn::Data::Enum(ref d) = ast.data {
        let mut variants = Vec::new();

        for v in &d.variants {
            let ident = &v.ident;
            let (_, lit) = v.discriminant.clone().ok_or(Error::new(
                ident.span(),
                format!("#[derive(FromRepr)] No variant for {ident}"),
            ))?;
            let is_default = find_named_attr(&v.attrs, "default").is_some();
            let display = if let Some(attr) = find_named_attr(&v.attrs, "display") {
                Some(parse_literal_from(attr)?)
            } else {
                None
            };

            variants.push((ident, lit, is_default, display));
        }

        if variants.is_empty() {
            return Err(Error::new(
                ast.ident.span(),
                "#[derive(FromRepr)] Found no variants".to_string(),
            ));
        }

        if variants.iter().filter(|(_, _, is_default, _)| *is_default).count() > 1 {
            return Err(Error::new(
                ast.ident.span(),
                "#[derive(FromRepr)] Found multiple default values".to_string(),
            ));
        }

        let default_variant = variants
            .iter()
            .find(|(_, _, is_default, _)| *is_default)
            .or(variants.first())
            .map(|(ident, _, _, _)| *ident)
            .unwrap();

        let from_rep_mapping: Vec<_> = variants
            .iter()
            .map(|(ident, lit, _, _)| {
                quote! { #lit => #name::#ident, }
            })
            .collect();

        let from_name_mapping: Vec<_> = variants
            .iter()
            .map(|(ident, lit, _, _)| {
                quote! { #name::#ident => #lit, }
            })
            .collect();

        let to_char_mapping: Vec<_> = variants
            .iter()
            .map(|(ident, lit, _, repr)| {
                if let Some(repr) = repr {
                    quote! { #name::#ident => #repr, }
                } else {
                    quote! { #name::#ident => #lit as char, }
                }
            })
            .collect();

        let parse_lines = variants.iter().map(|(ident, expr, _, _)| {
            quote! { #expr => Ok((rest, #name::#ident)), }
        });

        Ok(quote! {
            impl From<#rep> for #name {
                fn from(value: #rep) -> Self {
                    match value {
                        #(#from_rep_mapping)*
                        _ => #name::#default_variant,
                    }
                }
            }

            impl From<#name> for #rep {
                fn from(value: #name) -> Self {
                    match value {
                        #(#from_name_mapping)*
                    }
                }
            }

            impl From<#name> for char {
                fn from(value: #name) -> Self {
                    match value {
                        #(#to_char_mapping)*
                    }
                }
            }

            impl<I, E> nom_parse_trait::ParseFrom<I, E> for #name
            where
                E: nom::error::ParseError<I>,
                I: nom::Input + nom::AsBytes,
            {
                fn parse(input: I) -> nom::IResult<I, Self, E> {
                    if input.input_len() == 0 {
                        return Err(nom::Err::Error(E::from_error_kind(input, nom::error::ErrorKind::Eof)));
                    }

                    let (rest, value) = input.take_split(1);

                    match value.as_bytes()[0] {
                        #(#parse_lines)*
                        _ => Err(nom::Err::Error(E::from_error_kind(input, nom::error::ErrorKind::Char))),
                    }
                }
            }
        })
    } else {
        Err(Error::new(
            ast.ident.span(),
            "#[derive(FromRepr)] is only defined for Enum",
        ))
    }
}

fn parse_ident_from(attr: &Attribute) -> Result<Ident, Error> {
    match &attr.meta {
        Meta::Path(_) => Err(Error::new(
            attr.meta.span(),
            "Expected attribute list with some identifier",
        )),
        Meta::List(list) => {
            Ok(syn::parse(list.tokens.clone().into())
                .map_err(|err| Error::new(list.span(), err))?)
        }
        Meta::NameValue(namevalue) => {
            if let Expr::Path(expr) = &namevalue.value {
                expr.path
                    .get_ident()
                    .ok_or(Error::new(
                        expr.span(),
                        "Expression is not a valid identifier",
                    ))
                    .cloned()
            } else {
                Err(Error::new(
                    namevalue.value.span(),
                    "Expression is not a valid identifier",
                ))
            }
        }
    }
}

fn parse_literal_from(attr: &Attribute) -> Result<Lit, Error> {
    match &attr.meta {
        Meta::Path(_) => Err(Error::new(
            attr.meta.span(),
            "Expected attribute list with some literal value",
        )),
        Meta::List(list) => {
            Ok(syn::parse(list.tokens.clone().into())
                .map_err(|err| Error::new(list.span(), err))?)
        }
        Meta::NameValue(name_value) => {
            if let Expr::Lit(expr) = &name_value.value {
                Ok(expr.lit.clone())
            } else {
                Err(Error::new(
                    name_value.value.span(),
                    "Expression is not a valid literal value",
                ))
            }
        }
    }
}

fn find_named_attr<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|attr| attr.meta.path().is_ident(name))
}
