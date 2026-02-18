use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, Ident, LitStr};

use crate::ast::{
    Attribute, AttributeValue, ComponentCall, Element, ElseBranch, ForLoop, IfCondition,
    LetBinding, MatchArm, MatchExpression, NameValueAttribute, Node,
};

pub fn push_statements_for_node(statements: &mut Vec<TokenStream>, write: &Ident, node: Node) {
    match node {
        Node::Text(text) => push_statements_for_text(statements, write, text),
        Node::Expression(expr) => push_statements_for_expression(statements, write, expr),
        Node::RawExpression(expr) => push_statements_for_raw_expression(statements, write, expr),
        Node::Doctype(doctype) => push_statements_for_doctype(statements, write, doctype),
        Node::Children(children) => push_statements_for_children(statements, write, children),
        Node::LetBinding(let_binding) => push_statements_for_let_binding(statements, let_binding),
        Node::IfCondition(if_condition) => {
            push_statements_for_if_condition(statements, write, if_condition)
        }
        Node::ForLoop(for_loop) => push_statements_for_for_loop(statements, write, for_loop),
        Node::MatchExpression(match_expression) => {
            push_statements_for_match_expression(statements, write, match_expression)
        }
        Node::ComponentCall(component_call) => {
            push_statements_for_component_call(statements, write, component_call)
        }
        Node::HtmlDisplay(expr) => push_statements_for_html_display(statements, write, expr),
        Node::Element(element) => push_statements_for_element(statements, write, element),
    }
}

fn push_statements_for_text(statements: &mut Vec<TokenStream>, write: &Ident, text: LitStr) {
    statements.push(quote! {
        ::plait::display::HtmlDisplay::html_fmt(&::plait::display::Text(#text), #write)?;
    });
}

fn push_statements_for_expression(statements: &mut Vec<TokenStream>, write: &Ident, expr: Expr) {
    statements.push(quote! {
        ::plait::display::HtmlDisplay::html_fmt(&::plait::display::Text(#expr), #write)?;
    });
}

fn push_statements_for_raw_expression(
    statements: &mut Vec<TokenStream>,
    write: &Ident,
    expr: Expr,
) {
    statements.push(quote! {
        ::plait::display::HtmlDisplay::html_fmt(&::plait::display::Raw(#expr), #write)?;
    });
}

fn push_statements_for_doctype(statements: &mut Vec<TokenStream>, write: &Ident, doctype: Ident) {
    let lit = LitStr::new("<!DOCTYPE html>", doctype.span());

    statements.push(quote! {
        ::plait::display::HtmlDisplay::html_fmt(&::plait::display::Raw(#lit), #write)?;
    });
}

fn push_statements_for_children(statements: &mut Vec<TokenStream>, write: &Ident, children: Ident) {
    statements.push(quote! {
        #children(#write)?;
    });
}

fn push_statements_for_let_binding(statements: &mut Vec<TokenStream>, let_binding: LetBinding) {
    let LetBinding { pattern, expr } = let_binding;

    match expr {
        Some(expr) => statements.push(quote! {
            let #pattern = #expr;
        }),
        None => statements.push(quote! {
            let #pattern;
        }),
    }
}

fn push_statements_for_if_condition(
    statements: &mut Vec<TokenStream>,
    write: &Ident,
    if_condition: IfCondition,
) {
    let IfCondition {
        condition,
        then_branch,
        else_branch,
    } = if_condition;

    let mut then_statements = Vec::with_capacity(then_branch.len());

    for child in then_branch {
        push_statements_for_node(&mut then_statements, write, child);
    }

    match else_branch {
        None => statements.push(quote! {
            if #condition {
                #(#then_statements)*
            }
        }),
        Some(ElseBranch::Else(nodes)) => {
            let mut else_statements = Vec::with_capacity(nodes.len());

            for child in nodes {
                push_statements_for_node(&mut else_statements, write, child);
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

            push_statements_for_if_condition(&mut else_statements, write, *inner_if_condition);

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
    write: &Ident,
    for_loop: ForLoop,
) {
    let ForLoop {
        pattern,
        expression,
        body,
    } = for_loop;

    let mut body_statements = Vec::with_capacity(body.len());

    for child in body {
        push_statements_for_node(&mut body_statements, write, child);
    }

    statements.push(quote! {
        for #pattern in #expression {
            #(#body_statements)*
        }
    });
}

fn push_statements_for_match_expression(
    statements: &mut Vec<TokenStream>,
    write: &Ident,
    match_expression: MatchExpression,
) {
    let MatchExpression { expression, arms } = match_expression;

    let mut arm_statements = Vec::with_capacity(arms.len());

    for arm in arms {
        push_statement_for_match_arm(&mut arm_statements, write, arm);
    }

    statements.push(quote! {
        match #expression {
            #(#arm_statements)*
        }
    });
}

fn push_statement_for_match_arm(statements: &mut Vec<TokenStream>, write: &Ident, arm: MatchArm) {
    let MatchArm {
        pattern,
        guard,
        body,
    } = arm;

    let mut body_statements = Vec::new();

    for node in body {
        push_statements_for_node(&mut body_statements, write, node);
    }

    match guard {
        None => statements.push(quote! {
            #pattern => {
                #(#body_statements)*
            }
        }),
        Some(guard) => statements.push(quote! {
            #pattern if #guard => {
                #(#body_statements)*
            }
        }),
    }
}

fn push_statements_for_component_call(
    statements: &mut Vec<TokenStream>,
    write: &Ident,
    component_call: ComponentCall,
) {
    let mut field_statements = Vec::with_capacity(component_call.fields.len());

    for field in component_call.fields {
        let ident = field.ident;
        let value = field.value;

        field_statements.push(quote! {
            #ident : #value
        });
    }

    let path = component_call.path;

    let component_statement = quote! {
        &#path {
            #(#field_statements),*
        }
    };

    let mut attribute_statements = Vec::with_capacity(component_call.attributes.len());

    for attribute in component_call.attributes {
        push_statements_for_attribute(&mut attribute_statements, write, attribute);
    }

    let mut children_statements = Vec::with_capacity(component_call.children.len());

    for child in component_call.children {
        push_statements_for_node(&mut children_statements, write, child);
    }

    statements.push(quote! {
        ::plait::Component::html_fmt(
            #component_statement,
            #write,
            |#write: &mut (dyn ::core::fmt::Write + '_)| -> ::core::fmt::Result {
                #(#attribute_statements)*
                Ok(())
            },
            |#write: &mut (dyn ::core::fmt::Write + '_)| -> ::core::fmt::Result {
                #(#children_statements)*
                Ok(())
            },
        )?;
    });
}

fn push_statements_for_html_display(statements: &mut Vec<TokenStream>, write: &Ident, expr: Expr) {
    statements.push(quote! {
        ::plait::display::HtmlDisplay::html_fmt(#expr, #write)?;
    });
}

fn push_statements_for_element(statements: &mut Vec<TokenStream>, write: &Ident, element: Element) {
    let Element {
        name,
        is_void,
        attributes,
        children,
    } = element;

    statements.push(quote! {
        ::plait::display::HtmlDisplay::html_fmt(&::plait::display::OpenStartTag { name: #name }, #write)?;
    });

    for attribute in attributes {
        push_statements_for_attribute(statements, write, attribute);
    }

    statements.push(quote! {
        ::plait::display::HtmlDisplay::html_fmt(&::plait::display::CloseStartTag, #write)?;
    });

    if !is_void {
        for child in children {
            push_statements_for_node(statements, write, child);
        }

        statements.push(quote! {
            ::plait::display::HtmlDisplay::html_fmt(&::plait::display::CloseTag { name: #name }, #write)?;
        });
    }
}

fn push_statements_for_attribute(
    statements: &mut Vec<TokenStream>,
    write: &Ident,
    attribute: Attribute,
) {
    match attribute {
        Attribute::NameValue(attribute) => {
            let NameValueAttribute {
                name,
                is_maybe,
                is_url,
                value,
            } = attribute;

            match (is_maybe, value) {
                (false, Some(AttributeValue::Text(text))) => {
                    if is_url {
                        statements.push(quote! {
                            ::plait::display::HtmlDisplay::html_fmt(&::plait::display::UrlAttribute { name: #name, value: #text }, #write)?;
                        });
                    } else {
                        statements.push(quote! {
                            ::plait::display::HtmlDisplay::html_fmt(&::plait::display::Attribute { name: #name, value: #text }, #write)?;
                        });
                    }
                }
                (false, Some(AttributeValue::RawExpression(expr))) => {
                    statements.push(quote! {
                        ::plait::display::HtmlDisplay::html_fmt(&::plait::display::RawAttribute { name: #name, value: #expr }, #write)?;
                    });
                }
                (false, Some(AttributeValue::Expression(expr))) => {
                    if is_url {
                        statements.push(quote! {
                            ::plait::display::HtmlDisplay::html_fmt(&::plait::display::UrlAttribute { name: #name, value: #expr }, #write)?;
                        });
                    } else {
                        statements.push(quote! {
                            ::plait::display::HtmlDisplay::html_fmt(&::plait::display::Attribute { name: #name, value: #expr }, #write)?;
                        });
                    }
                }
                (false, None) => {
                    statements.push(quote! {
                        ::plait::display::HtmlDisplay::html_fmt(&::plait::display::MaybeAttribute { name: #name, value: true }, #write)?;
                    });
                }
                (true, Some(AttributeValue::Text(text))) => {
                    if is_url {
                        statements.push(quote! {
                            ::plait::display::HtmlDisplay::html_fmt(&::plait::display::UrlAttribute { name: #name, value: #text }, #write)?;
                        });
                    } else {
                        statements.push(quote! {
                            ::plait::display::HtmlDisplay::html_fmt(&::plait::display::Attribute { name: #name, value: #text }, #write)?;
                        });
                    }
                }
                (true, Some(AttributeValue::RawExpression(expr))) => {
                    statements.push(quote! {
                        ::plait::display::HtmlDisplay::html_fmt(&::plait::display::RawMaybeAttribute { name: #name, value: #expr }, #write)?;
                    });
                }
                (true, Some(AttributeValue::Expression(expr))) => {
                    if is_url {
                        statements.push(quote! {
                            ::plait::display::HtmlDisplay::html_fmt(&::plait::display::UrlMaybeAttribute { name: #name, value: #expr }, #write)?;
                        });
                    } else {
                        statements.push(quote! {
                            ::plait::display::HtmlDisplay::html_fmt(&::plait::display::MaybeAttribute { name: #name, value: #expr }, #write)?;
                        });
                    }
                }
                (true, None) => {}
            }
        }
        Attribute::Spread(attrs) => {
            statements.push(quote! {
                #attrs(#write)?;
            });
        }
    }
}
