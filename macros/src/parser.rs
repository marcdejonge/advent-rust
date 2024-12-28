use proc_macro2::{Ident, Span, TokenStream};
use syn::spanned::Spanned;
use syn::{Error, Expr, Field, Fields, ItemStruct, Meta, PathArguments, Type};

pub fn generate_struct_parser(
    parse_expression: Expr,
    item: &ItemStruct,
) -> Result<TokenStream, Error> {
    let name = &item.ident;
    let (generic_impl, generic_type, generic_where) = item.generics.split_for_impl();

    let fields: Vec<_> = item.fields.iter().filter_map(|f| f.ident.clone()).collect();
    let field_types: Vec<_> = item.fields.iter().map(|f| f.ty.clone()).collect();

    let map_line = if fields.is_empty() {
        let fields: Vec<_> = field_types
            .iter()
            .enumerate()
            .map(|(nr, _)| Ident::new(&format!("var_{}", nr), Span::call_site()))
            .collect();

        quote! { nom::combinator::map(parse_function, |(#(#fields),*) : (#(#field_types),*)| #name (#(#fields),*)) }
    } else {
        quote! { nom::combinator::map(parse_function, |(#(#fields),*) : (#(#field_types),*)| #name { #(#fields),* }) }
    };

    Ok(quote! {
        #item

        impl #generic_impl advent_lib::parsing::Parsable for #name #generic_type #generic_where {
            fn parser<'a>() -> impl nom::Parser<&'a [u8], Self, nom::error::Error<&'a [u8]>> {
                let parse_function = {
                    use advent_lib::parsing::*;
                    use nom::branch::*;
                    use nom::bytes::complete::*;
                    use nom::character::complete::*;
                    use nom::combinator::*;
                    use nom::multi::*;
                    use nom::sequence::*;
                    use nom::*;

                    #parse_expression
                };

                #map_line
            }
        }
    })
}

pub fn generate_std_struct_parser(item: &ItemStruct) -> Result<TokenStream, Error> {
    let name = &item.ident;

    if item.fields.is_empty() {
        Ok(quote! {
            #item

            impl advent_lib::parsing::Parsable for #name {
                fn parser<'a>() -> impl nom::Parser<&'a [u8], Self, nom::error::Error<&'a [u8]> {
                    #name
                }
            }
        })
    } else if item.fields.len() == 1 {
        let field = single_field(&item.fields)?;
        let field_name = get_field_name(&field)?;
        let field_type = strip_type(&field.ty)?;

        Ok(quote! {
            #item

            impl advent_lib::parsing::Parsable for #name {
                fn parser<'a>() -> impl nom::Parser<&'a [u8], Self, nom::error::Error<&'a [u8]>> {
                    nom::combinator::map(#field_type::parser(), |#field_name| #name { #field_name })
                }
            }
        })
    } else {
        Err(Error::new(
            item.span(),
            "Only structs with zero or one field are supported",
        ))
    }
}

fn strip_type(ty: &Type) -> Result<Type, Error> {
    Ok(if let Type::Path(mut path) = ty.clone() {
        path.path.segments.iter_mut().for_each(|s| {
            s.arguments = PathArguments::None;
        });
        Type::Path(path)
    } else {
        return Err(Error::new(ty.span(), "Expected a normal type"));
    })
}

fn get_field_name(field: &Field) -> Result<Ident, Error> {
    field
        .ident
        .clone()
        .ok_or(Error::new(field.span(), "Single field should have a name"))
}

fn single_field(item: &Fields) -> Result<Field, Error> {
    item.iter().next().cloned().ok_or(Error::new(
        item.span(),
        "Automating parsing requires a single field, multiple fields are not supported",
    ))
}

pub fn generate_enum_parser(item: &syn::ItemEnum) -> Result<TokenStream, Error> {
    let name = &item.ident;

    let mut filtered = item.clone();
    filtered.variants.iter_mut().for_each(|v| {
        v.attrs.retain(|attr| !attr.path().is_ident("format"));
    });

    let mut map_functions = Vec::new();
    let mut mappings = Vec::new();

    for variant in &item.variants {
        let attr =
            variant
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("format"))
                .ok_or(Error::new(
                    variant.span(),
                    "All variants of a parsable enum must have a #[format] attribute",
                ))?;

        match &attr.meta {
            Meta::Path(_) => {
                let var_name = &variant.ident;
                let field = single_field(&variant.fields)?;
                let full_type = field.ty.clone();
                let field_type = strip_type(&field.ty)?;

                mappings.push(quote! {
                    use advent_lib::parsing::*;

                    nom::combinator::map(
                        #field_type::parser(),
                        |value: #full_type| #name::#var_name(value)),
                });
            }
            Meta::List(_) => {
                return Err(Error::new(
                    attr.span(),
                    "The #[parsable] attribute must have either no arguments or an expression",
                ));
            }
            Meta::NameValue(nv) => {
                let var_name = &variant.ident;
                let expression = nv.value.clone();

                let function_name = Ident::new(&format!("map_{}", var_name), Span::call_site());
                map_functions.push(quote! { let #function_name = {
                    use advent_lib::parsing::*;
                    use nom::branch::*;
                    use nom::bytes::complete::*;
                    use nom::character::complete::*;
                    use nom::combinator::*;
                    use nom::multi::*;
                    use nom::sequence::*;
                    use nom::*;

                    #expression
                };
                });

                let value_types: Vec<_> = variant.fields.iter().map(|f| f.ty.clone()).collect();
                let value_names: Vec<_> = value_types
                    .iter()
                    .enumerate()
                    .map(|(nr, ty)| Ident::new(&format!("var_{}", nr), ty.span()))
                    .collect();

                if value_names.len() == 1 {
                    let value_expressions: Vec<_> = value_names
                        .iter()
                        .zip(value_types.iter())
                        .map(|(n, t)| {
                            quote! { #n: #t }
                        })
                        .collect();

                    mappings.push(quote! {
                        nom::combinator::map(
                            #function_name,
                            |#(#value_expressions),*| #name::#var_name(#(#value_names),*)
                        )
                    });
                } else {
                    mappings.push(quote! {
                        nom::combinator::map(
                            #function_name,
                            |(#(#value_names),*): (#(#value_types),*)| #name::#var_name(#(#value_names),*)
                        )
                    });
                }
            }
        }
    }

    Ok(quote! {
        #filtered

        impl advent_lib::parsing::Parsable for #name {
            fn parser<'a>() -> impl nom::Parser<&'a [u8], Self, nom::error::Error<&'a [u8]>> {
                #(#map_functions)*
                nom::branch::alt((
                    #(#mappings),*
                ))
            }
        }
    })
}
