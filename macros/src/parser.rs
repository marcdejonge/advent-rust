use proc_macro2::{Ident, Span, TokenStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Error, Expr, Field, Fields, FieldsNamed, ItemStruct, Meta, PathArguments, Type};

pub fn generate_struct_parser(
    parse_expression: Option<Expr>,
    item: &ItemStruct,
) -> Result<TokenStream, Error> {
    let name = &item.ident;
    let (generic_impl, generic_type, generic_where) = item.generics.split_for_impl();

    let field_info = parse_field_info(&item.fields)?;
    let expression = generate_expression(parse_expression, &field_info.types)?;

    let create_expression = field_info.create_expression;

    let field_names = &field_info.names;
    let field_types = &field_info.types;

    let imports = generate_imports();

    let mut item = item.clone();
    item.fields = field_info.fields;

    Ok(quote! {
        #item

        impl #generic_impl advent_lib::parsing::Parsable for #name #generic_type #generic_where {
            fn parser<'a>() -> impl nom::Parser<&'a [u8], Self, nom::error::Error<&'a [u8]>> {
                let parse_function = { #imports #expression };

                nom::combinator::map(
                    parse_function,
                    |(#(#field_names),*) : (#(#field_types),*)| #name #create_expression
                )
            }
        }
    })
}

struct FieldInfo {
    fields: Fields,
    names: Vec<Ident>,
    types: Vec<Type>,
    create_expression: TokenStream,
}

impl FieldInfo {
    fn new() -> Self {
        Self {
            fields: Fields::Unit,
            names: Vec::new(),
            types: Vec::new(),
            create_expression: TokenStream::new(),
        }
    }
}

fn parse_field_info(fields: &Fields) -> Result<FieldInfo, Error> {
    let mut field_info = FieldInfo::new();

    match fields {
        Fields::Named(FieldsNamed { brace_token, named: fields, .. }) => {
            let mut new_fields = Punctuated::new();
            let mut expressions: Vec<TokenStream> = Vec::new();

            for field in fields {
                if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("defer")) {
                    let expression: Expr = attr.parse_args()?;
                    let mut name = field.ident.clone().unwrap();
                    name.set_span(attr.path().span());

                    let mut field = field.clone();
                    field.attrs.retain(|attr| !attr.path().is_ident("defer"));
                    new_fields.push(field);

                    expressions.insert(0, quote! { #name: #expression });
                } else if field.attrs.iter().any(|attr| attr.path().is_ident("intermediate")) {
                    field_info.names.push(field.ident.clone().unwrap());
                    field_info.types.push(field.ty.clone());
                } else {
                    field_info.names.push(field.ident.clone().unwrap());
                    field_info.types.push(field.ty.clone());
                    new_fields.push(field.clone());

                    let name = field.ident.clone().unwrap();
                    expressions.push(quote! { #name });
                }
            }

            field_info.fields =
                Fields::Named(FieldsNamed { brace_token: *brace_token, named: new_fields });
            field_info.create_expression = quote! { { #(#expressions),* } };
        }
        Fields::Unnamed(syn::FieldsUnnamed { unnamed: unnamed_fields, .. }) => {
            let mut expressions: Vec<TokenStream> = Vec::new();

            for (ix, field) in unnamed_fields.iter().enumerate() {
                let name = Ident::new(&format!("var_{}", ix), field.ty.span());
                field_info.names.push(name.clone());
                expressions.push(quote! { #name });
                field_info.types.push(field.ty.clone());
            }

            field_info.fields = fields.clone();
            field_info.create_expression = quote! { ( #(#expressions),* ) };
        }
        Fields::Unit => {}
    };

    if field_info.names.is_empty() && field_info.types.is_empty() {
        field_info.names.push(Ident::new("it", Span::call_site()));
        field_info.types.push(syn::parse_quote! { _ });
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
        let attr = variant.attrs.iter().find(|attr| attr.path().is_ident("format"));

        match attr.map(|attr| &attr.meta) {
            None | Some(Meta::Path(_)) => {
                if variant.fields.is_empty() {
                    let ident = &variant.ident;
                    let var_name = syn::LitByteStr::new(ident.to_string().as_bytes(), ident.span());

                    mappings.push(quote! {
                        nom::combinator::value(
                            #name::#ident,
                            nom::bytes::complete::tag(#var_name)
                        )
                    });
                } else {
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
            }
            Some(Meta::List(_)) => {
                return Err(Error::new(
                    attr.span(),
                    "The #[parsable] attribute must have either no arguments or an expression",
                ));
            }
            Some(Meta::NameValue(nv)) => {
                let field_info = parse_field_info(&variant.fields)?;
                let var_name = &variant.ident;
                let expression = nv.value.clone();

                let function_name = Ident::new(&format!("map_{}", var_name), Span::call_site());
                let imports = generate_imports();
                map_functions.push(quote! { let #function_name = { #imports #expression }; });

                let create_expression = field_info.create_expression;
                let field_names = &field_info.names;
                let field_types = &field_info.types;

                mappings.push(quote! {
                    nom::combinator::map(
                        #function_name,
                        |(#(#field_names),*) : (#(#field_types),*)| #name::#var_name #create_expression
                    )
                });
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
        use nom::character::*;
        use nom::character::complete::*;
        use nom::combinator::*;
        use nom::multi::*;
        use nom::sequence::*;
        use nom::*;
    }
}
