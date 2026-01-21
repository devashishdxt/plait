use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

use crate::ast::{Attribute, AttributeValue, EscapeMode};

struct AttrsInput {
    attributes: Vec<Attribute>,
}

impl Parse for AttrsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        parse_attrs_input(input)
    }
}

fn parse_attrs_input(input: ParseStream<'_>) -> syn::Result<AttrsInput> {
    let mut attributes = Vec::new();
    while !input.is_empty() {
        attributes.push(input.parse()?);
    }
    Ok(AttrsInput { attributes })
}

/// Implementation of the `attrs!` procedural macro.
///
/// Parses the input tokens as a list of attributes and generates code that
/// constructs an [`Attributes`](plait::Attributes) collection at runtime.
pub fn attrs_impl(input: TokenStream) -> TokenStream {
    let attrs_input: AttrsInput = match syn::parse2(input) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error(),
    };

    let mut add_calls = Vec::new();

    for attribute in attrs_input.attributes {
        match attribute {
            Attribute::NameValue { name, value } => {
                let name = name.name;

                match value {
                    Some(AttributeValue::Literal { value }) => add_calls.push(quote! {
                        __spool_attrs.add( #name, #value, ::core::option::Option::Some(::plait::EscapeMode::Raw) );
                    }),
                    Some(AttributeValue::Dynamic { expr }) => {
                        let escape_mode = match expr.escape_mode {
                            None => quote! {
                                ::core::option::Option::None
                            },
                            Some(EscapeMode::Html) => quote! {
                                ::core::option::Option::Some(::plait::EscapeMode::Html)
                            },
                            Some(EscapeMode::Raw) => quote! {
                                ::core::option::Option::Some(::plait::EscapeMode::Raw)
                            },
                        };
                        let expr = expr.expr;

                        add_calls.push(quote! {
                            __spool_attrs.add( #name, #expr, #escape_mode );
                        });
                    },
                    Some(AttributeValue::Optional { expr }) => {
                        let escape_mode = match expr.escape_mode {
                            None => quote! {
                                ::core::option::Option::None
                            },
                            Some(EscapeMode::Html) => quote! {
                                ::core::option::Option::Some(::plait::EscapeMode::Html)
                            },
                            Some(EscapeMode::Raw) => quote! {
                                ::core::option::Option::Some(::plait::EscapeMode::Raw)
                            },
                        };
                        let expr = expr.expr;

                        add_calls.push(quote! {
                            __spool_attrs.add_optional( #name, #expr, #escape_mode );
                        });
                    },
                    Some(AttributeValue::Boolean { expr }) => add_calls.push(quote! {
                        __spool_attrs.add_boolean( #name, #expr );
                    }),
                    None => add_calls.push(quote! {
                        __spool_attrs.add_boolean( #name, true );
                    }),
                }
            }
            Attribute::Spread { expr } => add_calls.push(quote! {
                __spool_attrs.merge( #expr );
            }),
        }
    }

    let capacity = add_calls.len();

    quote! {
        {
            let mut __spool_attrs = ::plait::Attributes::with_capacity(#capacity);
            #(#add_calls)*
            __spool_attrs
        }
    }
}
