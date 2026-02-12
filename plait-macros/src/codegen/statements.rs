use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::ast::{
    Attribute, AttributeValue, ComponentCall, Element, ElseBranch, ForLoop, IfCondition, MatchArm,
    MatchExpression, Node,
};

pub fn push_statements_for_node(statements: &mut Vec<TokenStream>, formatter: &Ident, node: Node) {
    match node {
        Node::Text(text) => {
            statements.push(quote! {
                #formatter .write_html_escaped(#text);
            });
        }
        Node::RawExpression(expr) => {
            statements.push(quote! {
                ::plait::IntoHtmlRaw::render_raw_to(#expr, #formatter);
            });
        }
        Node::Expression(expr) => {
            statements.push(quote! {
                ::plait::IntoHtml::render_to(#expr, #formatter);
            });
        }
        Node::Children(children) => {
            statements.push(quote! {
                #children ( #formatter );
            });
        }
        Node::If(if_condition) => {
            push_statements_for_if_condition(statements, formatter, if_condition)
        }
        Node::For(for_loop) => push_statements_for_for_loop(statements, formatter, for_loop),
        Node::Match(match_expression) => {
            push_statements_for_match_expression(statements, formatter, match_expression)
        }
        Node::Component(component_call) => {
            push_statements_for_component_call(statements, formatter, component_call)
        }
        Node::Element(element) => {
            push_statements_for_element(statements, formatter, element);
        }
    }
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

fn push_statements_for_match_expression(
    statements: &mut Vec<TokenStream>,
    formatter: &Ident,
    match_expression: MatchExpression,
) {
    let expression = match_expression.expression;
    let arms = match_expression.arms;

    let mut arm_statements = Vec::with_capacity(arms.len());

    for arm in arms {
        push_statement_for_match_arm(&mut arm_statements, formatter, arm);
    }

    statements.push(quote! {
        match #expression {
            #(#arm_statements)*
        }
    });
}

fn push_statement_for_match_arm(
    statements: &mut Vec<TokenStream>,
    formatter: &Ident,
    arm: MatchArm,
) {
    let pattern = arm.pattern;
    let guard = arm.guard;
    let body = arm.body;

    let mut body_statements = Vec::new();

    for node in body {
        push_statements_for_node(&mut body_statements, formatter, node);
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
    formatter: &Ident,
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
        #path {
            #(#field_statements),*
        }
    };

    let mut attribute_statements = Vec::with_capacity(component_call.attributes.len());

    for attribute in component_call.attributes {
        push_statements_for_attribute(&mut attribute_statements, formatter, attribute);
    }

    let mut children_statements = Vec::with_capacity(component_call.children.len());

    for child in component_call.children {
        push_statements_for_node(&mut children_statements, formatter, child);
    }

    statements.push(quote! {
        ::plait::Component::render(
            #component_statement,
            #formatter,
            |#formatter: &mut ::plait::HtmlFormatter<'_>| {
                #(#attribute_statements)*
            },
            |#formatter: &mut ::plait::HtmlFormatter<'_>| {
                #(#children_statements)*
            },
        );
    });
}

fn push_statements_for_element(
    statements: &mut Vec<TokenStream>,
    formatter: &Ident,
    element: Element,
) {
    let name = element.name;

    statements.push(quote! {
        #formatter .open_tag(#name);
    });

    for attribute in element.attributes {
        push_statements_for_attribute(statements, formatter, attribute);
    }

    statements.push(quote! {
        #formatter .close_start_tag();
    });

    if !element.is_void {
        for child in element.children {
            push_statements_for_node(statements, formatter, child);
        }

        statements.push(quote! {
            #formatter .close_tag(#name);
        });
    }
}

fn push_statements_for_attribute(
    statements: &mut Vec<TokenStream>,
    formatter: &Ident,
    attribute: Attribute,
) {
    match attribute {
        Attribute::NameValue(attribute) => {
            let attribute_name = attribute.name;

            match (attribute.is_maybe, attribute.value) {
                (false, Some(AttributeValue::Text(text))) => {
                    if attribute.is_url {
                        statements.push(quote! {
                            #formatter .write_url_attribute_escaped(#attribute_name, #text);
                        });
                    } else {
                        statements.push(quote! {
                            #formatter .write_attribute_escaped(#attribute_name, #text);
                        });
                    }
                }
                (false, Some(AttributeValue::RawExpression(expr))) => {
                    statements.push(quote! {
                        #formatter .write_attribute_raw(#attribute_name, #expr);
                    });
                }
                (false, Some(AttributeValue::Expression(expr))) => {
                    if attribute.is_url {
                        statements.push(quote! {
                            #formatter .write_url_attribute_escaped(#attribute_name, #expr);
                        });
                    } else {
                        statements.push(quote! {
                            #formatter .write_attribute_escaped(#attribute_name, #expr);
                        });
                    }
                }
                (false, None) => {
                    statements.push(quote! {
                        #formatter .write_boolean_attribute(#attribute_name, true);
                    });
                }
                (true, Some(AttributeValue::Text(text))) => {
                    if attribute.is_url {
                        statements.push(quote! {
                            #formatter .write_url_attribute_escaped(#attribute_name, #text);
                        });
                    } else {
                        statements.push(quote! {
                            #formatter .write_attribute_escaped(#attribute_name, #text);
                        });
                    }
                }
                (true, Some(AttributeValue::RawExpression(expr))) => {
                    statements.push(quote! {
                        #formatter .write_maybe_attribute_raw(#attribute_name, #expr);
                    });
                }
                (true, Some(AttributeValue::Expression(expr))) => {
                    if attribute.is_url {
                        statements.push(quote! {
                            #formatter .write_optional_url_attribute_escaped(#attribute_name, #expr);
                        });
                    } else {
                        statements.push(quote! {
                            #formatter .write_maybe_attribute_escaped(#attribute_name, #expr);
                        });
                    }
                }
                (true, None) => unreachable!(),
            }
        }
        Attribute::Spread(attrs) => {
            statements.push(quote! {
                #attrs ( #formatter  );
            });
        }
    }
}
