use proc_macro2::{Ident, Span, TokenStream};
use syn::spanned::Spanned;
use syn::{Error, Expr, Field, Fields, ItemStruct, Meta, PathArguments, Type};

pub fn generate_struct_parser(
    parse_expression: Option<Expr>,
    item: &ItemStruct,
) -> Result<TokenStream, Error> {
    let mut filtered = item.clone();
    filtered.fields.iter_mut().for_each(|field| {
        field.attrs.retain(|attr| !attr.path().is_ident("defer"));
    });

    let name = &item.ident;
    let (generic_impl, generic_type, generic_where) = item.generics.split_for_impl();

    let field_info = parse_field_info(item)?;
    let expression = generate_expression(parse_expression, &field_info.types)?;

    let map_line = if field_info.names.is_empty() {
        let field_names = field_info.generate_anonymous_names();
        let field_types = field_info.types;

        quote! { nom::combinator::map(parse_function, |(#(#field_names),*) : (#(#field_types),*)|
            #name (#(#field_names),*))
        }
    } else {
        let mut field_expressions = field_info.deferred;
        field_expressions.extend(field_info.names.iter().map(|n| quote! { #n }));
        let field_names = &field_info.names;
        let field_types = &field_info.types;

        quote! { nom::combinator::map(parse_function, |(#(#field_names),*) : (#(#field_types),*)|
            #name { #(#field_expressions),* })
        }
    };

    let imports = generate_imports();

    Ok(quote! {
        #filtered

        impl #generic_impl advent_lib::parsing::Parsable for #name #generic_type #generic_where {
            fn parser<'a>() -> impl nom::Parser<&'a [u8], Self, nom::error::Error<&'a [u8]>> {
                let parse_function = { #imports #expression };
                #map_line
            }
        }
    })
}

#[derive(Default)]
struct FieldInfo {
    names: Vec<Ident>,
    types: Vec<Type>,
    deferred: Vec<TokenStream>,
}

impl FieldInfo {
    fn generate_anonymous_names(&self) -> Vec<Ident> {
        self.types
            .iter()
            .enumerate()
            .map(|(nr, _)| Ident::new(&format!("var_{}", nr), Span::call_site()))
            .collect()
    }
}

fn parse_field_info(item: &ItemStruct) -> Result<FieldInfo, Error> {
    let mut field_info = FieldInfo::default();

    for field in &item.fields {
        if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("defer")) {
            let expression: Expr = attr.parse_args()?;
            let mut name = field.ident.clone().unwrap();
            name.set_span(attr.path().span());
            field_info.deferred.push(quote! { #name: #expression });
        } else {
            if let Some(ident) = &field.ident {
                field_info.names.push(ident.clone());
            }
            field_info.types.push(field.ty.clone());
        }
    }

    Ok(field_info)
}

fn generate_expression(
    expression: Option<Expr>,
    field_types: &[Type],
) -> Result<TokenStream, Error> {
    if let Some(expr) = expression {
        Ok(quote! { #expr })
    } else if field_types.len() == 1 {
        Ok(generate_parser(&field_types[0]))
    } else {
        let field_parsers: Vec<_> = field_types.iter().map(generate_parser).collect();
        Ok(quote! { tuple((#(#field_parsers),*)) })
    }
}

fn generate_parser(field_type: &Type) -> TokenStream {
    if let Type::Path(path) = field_type {
        if let Some(segment) = path.path.segments.first() {
            if segment.ident == "Vec" {
                return quote! { separated_lines1() };
            }
        }
    }

    let stripped = strip_type(field_type);
    quote! { #stripped::parser() }
}

fn strip_type(ty: &Type) -> Type {
    if let Type::Path(mut path) = ty.clone() {
        path.path.segments.iter_mut().for_each(|s| {
            s.arguments = PathArguments::None;
        });
        Type::Path(path)
    } else {
        ty.clone()
    }
}

fn single_field(item: &Fields) -> Result<Field, Error> {
    item.iter().next().cloned().ok_or(Error::new(
        item.span(),
        "Automating parsing requires a single field, multiple fields are not supported",
    ))
}

pub fn generate_enum_parser(item: &syn::ItemEnum) -> Result<TokenStream, Error> {
    let mut name = item.ident.clone();
    name.set_span(Span::call_site());

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
                let field_type = strip_type(&field.ty);

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
                let imports = generate_imports();
                map_functions.push(quote! { let #function_name = { #imports #expression }; });

                let value_types: Vec<_> = variant.fields.iter().map(|f| f.ty.clone()).collect();
                let value_names: Vec<_> = value_types
                    .iter()
                    .enumerate()
                    .map(|(nr, ty)| Ident::new(&format!("var_{}", nr), ty.span()))
                    .collect();

                if value_names.is_empty() {
                    mappings.push(quote! {
                        nom::combinator::map(
                            #function_name,
                            |_| #name::#var_name
                        )
                    });
                } else if value_names.len() == 1 {
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

fn generate_imports() -> TokenStream {
    quote! {
        use advent_lib::parsing::*;
        use nom::branch::*;
        use nom::bytes::complete::*;
        use nom::character::complete::*;
        use nom::combinator::*;
        use nom::multi::*;
        use nom::sequence::*;
        use nom::*;
    }
}
