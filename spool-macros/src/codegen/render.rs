use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident,
    parse::{Parse, ParseStream},
    token::Comma,
};

use crate::ast::{
    Attribute, AttributeValue, Element, ElseBranch, EscapeMode, ForLoop, IfCondition, Node,
};

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
        Node::If(if_condition) => {
            push_statements_for_if_condition(statements, formatter, if_condition);
        }
        Node::For(for_loop) => {
            push_statements_for_for_loop(statements, formatter, for_loop);
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

fn push_statements_for_if_condition(
    statements: &mut Vec<TokenStream>,
    formatter: &Ident,
    if_condition: IfCondition,
) {
    let condition = if_condition.condition;
    let then_branch = if_condition.then_branch;

    let mut then_statements = Vec::with_capacity(then_branch.len());

    for child in then_branch {
        push_statements_for_node(&mut then_statements, formatter, child);
    }

    match if_condition.else_branch {
        None => statements.push(quote! {
            if #condition {
                #(#then_statements)*
            }
        }),
        Some(ElseBranch::Else(nodes)) => {
            let mut else_statements = Vec::with_capacity(nodes.len());

            for child in nodes {
                push_statements_for_node(&mut else_statements, formatter, child);
            }

            statements.push(quote! {
                if #condition {
                    #(#then_statements)*
                } else {
                    #(#else_statements)*
                }
            });
        }
        Some(ElseBranch::If(inner_if_condition)) => {
            let mut else_statements = Vec::new();

            push_statements_for_if_condition(&mut else_statements, formatter, *inner_if_condition);

            statements.push(quote! {
                if #condition {
                    #(#then_statements)*
                } else {
                    #(#else_statements)*
                }
            });
        }
    }
}

fn push_statements_for_for_loop(
    statements: &mut Vec<TokenStream>,
    formatter: &Ident,
    for_loop: ForLoop,
) {
    let pattern = for_loop.pattern;
    let expression = for_loop.expression;
    let body = for_loop.body;

    let mut body_statements = Vec::with_capacity(body.len());

    for child in body {
        push_statements_for_node(&mut body_statements, formatter, child);
    }

    statements.push(quote! {
        for #pattern in #expression {
            #(#body_statements)*
        }
    });
}
