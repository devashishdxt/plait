use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident,
    parse::{Parse, ParseStream},
    token::Comma,
};

use crate::ast::{Attribute, AttributeValue, Element, EscapeMode, Node};

struct RenderInput {
    formatter: Ident,
    node: Node,
}

impl Parse for RenderInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_html_input(input)
    }
}

fn parse_html_input(input: ParseStream<'_>) -> syn::Result<RenderInput> {
    let formatter = input.parse()?;
    let _: Comma = input.parse()?;
    let node = input.parse()?;

    Ok(RenderInput { formatter, node })
}

pub fn render_impl(input: TokenStream) -> TokenStream {
    let render_input: RenderInput = match syn::parse2(input) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error(),
    };

    let mut statements = Vec::new();

    push_statements_for_node(&mut statements, &render_input.formatter, render_input.node);

    quote! {
        {
            #(#statements)*
        }
    }
}

fn push_statements_for_node(statements: &mut Vec<TokenStream>, formatter: &Ident, node: Node) {
    match node {
        Node::Text(text) => {
            statements.push(quote! {
                #formatter .write_content(#text, ::core::option::Option::Some(::spool::EscapeMode::Raw)).unwrap();
            });
        }
        Node::Expression(expr) => {
            let escape_mode = match expr.escape_mode {
                None => quote! {
                    ::core::option::Option::None
                },
                Some(EscapeMode::Html) => quote! {
                    ::core::option::Option::Some(::spool::EscapeMode::Html)
                },
                Some(EscapeMode::Raw) => quote! {
                    ::core::option::Option::Some(::spool::EscapeMode::Raw)
                },
            };
            let expr = expr.expr;

            statements.push(quote! {
                #formatter .write_content(#expr, #escape_mode).unwrap();
            });
        }
        Node::Fragment(nodes) => {
            for node in nodes {
                push_statements_for_node(statements, formatter, node);
            }
        }
        Node::Element(element) => {
            push_statements_for_element(statements, formatter, element);
        }
    }
}

fn push_statements_for_element(
    statements: &mut Vec<TokenStream>,
    formatter: &Ident,
    element: Element,
) {
    let element_name = element.name;

    statements.push(quote! {
        #formatter .start_element(#element_name);
    });

    for attribute in element.attributes {
        match attribute {
            Attribute::NameValue { name, value } => {
                let name = name.name;

                match value {
                    Some(AttributeValue::Literal { value }) => statements.push(quote! {
                        #formatter .write_attribute( #name, #value, ::core::option::Option::Some(::spool::EscapeMode::Raw) ).unwrap();
                    }),
                    Some(AttributeValue::Dynamic { expr }) => {
                        let escape_mode = match expr.escape_mode {
                            None => quote! {
                                ::core::option::Option::None
                            },
                            Some(EscapeMode::Html) => quote! {
                                ::core::option::Option::Some(::spool::EscapeMode::Html)
                            },
                            Some(EscapeMode::Raw) => quote! {
                                ::core::option::Option::Some(::spool::EscapeMode::Raw)
                            },
                        };
                        let expr = expr.expr;

                        statements.push(quote! {
                            #formatter .write_attribute( #name, #expr, #escape_mode ).unwrap();
                        });
                    },
                    Some(AttributeValue::Optional { expr }) => {
                        let escape_mode = match expr.escape_mode {
                            None => quote! {
                                ::core::option::Option::None
                            },
                            Some(EscapeMode::Html) => quote! {
                                ::core::option::Option::Some(::spool::EscapeMode::Html)
                            },
                            Some(EscapeMode::Raw) => quote! {
                                ::core::option::Option::Some(::spool::EscapeMode::Raw)
                            },
                        };
                        let expr = expr.expr;

                        statements.push(quote! {
                            #formatter .write_optional_attribute( #name, #expr, #escape_mode ).unwrap();
                        });
                    },
                    Some(AttributeValue::Boolean { expr }) => statements.push(quote! {
                        #formatter .write_boolean_attribute( #name, #expr ).unwrap();
                    }),
                    None => statements.push(quote! {
                        #formatter .write_boolean_attribute( #name, true ).unwrap();
                    }),
                }
            }
            Attribute::Spread { expr } => statements.push(quote! {
                #formatter .spread_attributes( #expr ).unwrap();
            }),
        }
    }

    for child in element.children {
        push_statements_for_node(statements, formatter, child);
    }

    statements.push(quote! {
        #formatter .end_element().unwrap();
    });
}
