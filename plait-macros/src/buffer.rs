use std::{
    cmp::max,
    ops::{Deref, DerefMut},
};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, Ident, Lit, LitBool, LitChar, LitFloat, LitInt, LitStr, spanned::Spanned};

use crate::{
    ast::{
        Attribute, AttributeValue, ComponentCall, Element, ElseBranch, ForLoop, IfCondition,
        LetBinding, MatchArm, MatchExpression, Node,
    },
    utils::{escape_html_to, is_void_element},
};

pub struct Buffer {
    pub input_size: usize,
    pub inner: InnerBuffer,
}

impl Buffer {
    pub fn new(input: &TokenStream) -> Self {
        Self {
            input_size: input.to_string().len(),
            inner: InnerBuffer::new(Ident::new("__plait_html", input.span())),
        }
    }

    pub fn finalize_html(mut self) -> TokenStream {
        self.flush_static_str();

        let InnerBuffer {
            writer,
            static_str: _,
            size_hint,
            token_stream,
            has_dynamic_value,
        } = self.inner;

        let size_hint = if has_dynamic_value {
            max(size_hint, self.input_size)
        } else {
            size_hint
        };

        quote! {
            ::plait::HtmlFragment::new(
                move |#writer: &mut (dyn ::core::fmt::Write + '_)| -> ::core::fmt::Result {
                    #token_stream
                    Ok(())
                },
                #size_hint,
            )
        }
    }
}

impl Deref for Buffer {
    type Target = InnerBuffer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct InnerBuffer {
    pub writer: Ident,
    pub static_str: String,
    pub size_hint: usize,
    pub token_stream: TokenStream,
    pub has_dynamic_value: bool,
}

impl InnerBuffer {
    pub fn new(writer: Ident) -> Self {
        InnerBuffer {
            writer,
            static_str: String::new(),
            size_hint: 0,
            token_stream: TokenStream::new(),
            has_dynamic_value: false,
        }
    }

    pub fn push_block(&mut self, block: &[Node]) {
        for child in block {
            self.push_node(child);
        }
    }

    fn push_node(&mut self, node: &Node) {
        match node {
            Node::Doctype => self.push_doctype(),
            Node::LitStr(lit_str) => self.push_lit_str_escaped(lit_str),
            Node::LitChar(lit_char) => self.push_lit_char_escaped(lit_char),
            Node::LitInt(lit_int) => self.push_lit_int(lit_int),
            Node::LitFloat(lit_float) => self.push_lit_float(lit_float),
            Node::LitBool(lit_bool) => self.push_lit_bool(lit_bool),
            Node::Escaped(expr) => self.push_expr_escaped(expr),
            Node::Raw(expr) => self.push_expr_raw(expr),
            Node::LetBinding(let_binding) => self.push_let_binding(let_binding),
            Node::IfCondition(if_condition) => self.push_if_condition(if_condition),
            Node::MatchExpression(match_expression) => self.push_match_expression(match_expression),
            Node::ForLoop(for_loop) => self.push_for_loop(for_loop),
            Node::Element(element) => self.push_element(element),
            Node::Block(block) => self.push_block(block),
            Node::Children(children) => self.push_children(children),
            Node::ComponentCall(component_call) => self.push_component_call(component_call),
        }
    }

    fn push_doctype(&mut self) {
        self.static_str.push_str("<!DOCTYPE html>");
    }

    fn push_lit_str_escaped(&mut self, lit_str: &LitStr) {
        escape_html_to(&mut self.static_str, &lit_str.value());
    }

    fn push_lit_str_raw(&mut self, lit_str: &LitStr) {
        self.static_str.push_str(&lit_str.value());
    }

    fn push_lit_char_escaped(&mut self, lit_char: &LitChar) {
        escape_html_to(&mut self.static_str, &lit_char.value().to_string());
    }

    fn push_lit_char_raw(&mut self, lit_char: &LitChar) {
        self.static_str.push(lit_char.value());
    }

    fn push_lit_int(&mut self, lit_int: &LitInt) {
        self.static_str.push_str(lit_int.base10_digits());
    }

    fn push_lit_float(&mut self, lit_float: &LitFloat) {
        self.static_str.push_str(lit_float.base10_digits());
    }

    fn push_lit_bool(&mut self, lit_bool: &LitBool) {
        self.static_str
            .push_str(if lit_bool.value() { "true" } else { "false" });
    }

    fn push_expr_escaped(&mut self, expr: &Expr) {
        match expr {
            Expr::Lit(expr_lit) => match &expr_lit.lit {
                Lit::Str(lit_str) => self.push_lit_str_escaped(lit_str),
                Lit::Char(lit_char) => self.push_lit_char_escaped(lit_char),
                Lit::Int(lit_int) => self.push_lit_int(lit_int),
                Lit::Float(lit_float) => self.push_lit_float(lit_float),
                Lit::Bool(lit_bool) => self.push_lit_bool(lit_bool),
                _ => self.push_dynamic_expr_escaped(expr),
            },
            _ => self.push_dynamic_expr_escaped(expr),
        }
    }

    fn push_expr_raw(&mut self, expr: &Expr) {
        match expr {
            Expr::Lit(expr_lit) => match &expr_lit.lit {
                Lit::Str(lit_str) => self.push_lit_str_raw(lit_str),
                Lit::Char(lit_char) => self.push_lit_char_raw(lit_char),
                Lit::Int(lit_int) => self.push_lit_int(lit_int),
                Lit::Float(lit_float) => self.push_lit_float(lit_float),
                Lit::Bool(lit_bool) => self.push_lit_bool(lit_bool),
                _ => self.push_dynamic_expr_raw(expr),
            },
            _ => self.push_dynamic_expr_raw(expr),
        }
    }

    fn push_let_binding(&mut self, let_binding: &LetBinding) {
        self.flush_static_str();

        let LetBinding { pattern, expr } = let_binding;

        match expr {
            Some(expr) => self.token_stream.extend(quote! {
                let #pattern = #expr;
            }),
            None => self.token_stream.extend(quote! {
                let #pattern;
            }),
        }
    }

    fn push_if_condition(&mut self, if_condition: &IfCondition) {
        self.flush_static_str();

        let IfCondition {
            condition,
            then_branch,
            else_branch,
        } = if_condition;

        let mut then_buffer = self.create_inner();
        then_buffer.push_block(then_branch);
        then_buffer.flush_static_str();

        match else_branch {
            None => {
                let then_branch = then_buffer.token_stream;

                self.token_stream.extend(quote! {
                    if #condition {
                        #then_branch
                    }
                });

                self.has_dynamic_value = self.has_dynamic_value || then_buffer.has_dynamic_value;
                self.size_hint += then_buffer.size_hint;
            }
            Some(ElseBranch::Else(else_branch)) => {
                let mut else_buffer = self.create_inner();
                else_buffer.push_block(else_branch);
                else_buffer.flush_static_str();

                let then_branch = then_buffer.token_stream;
                let else_branch = else_buffer.token_stream;

                self.token_stream.extend(quote! {
                    if #condition {
                        #then_branch
                    } else {
                        #else_branch
                    }
                });

                self.has_dynamic_value = self.has_dynamic_value
                    || then_buffer.has_dynamic_value
                    || else_buffer.has_dynamic_value;
                self.size_hint += max(then_buffer.size_hint, else_buffer.size_hint);
            }
            Some(ElseBranch::If(else_if_branch)) => {
                let mut else_buffer = self.create_inner();

                else_buffer.push_if_condition(&else_if_branch);
                else_buffer.flush_static_str();

                let then_branch = then_buffer.token_stream;
                let else_branch = else_buffer.token_stream;

                self.token_stream.extend(quote! {
                    if #condition {
                        #then_branch
                    } else {
                        #else_branch
                    }
                });

                self.has_dynamic_value = self.has_dynamic_value
                    || then_buffer.has_dynamic_value
                    || else_buffer.has_dynamic_value;
                self.size_hint += max(then_buffer.size_hint, else_buffer.size_hint);
            }
        }
    }

    fn push_match_expression(&mut self, match_expression: &MatchExpression) {
        self.flush_static_str();

        let MatchExpression { expression, arms } = match_expression;

        let mut arms_buffer = self.create_inner();

        for arm in arms {
            let MatchArm {
                pattern,
                guard,
                body,
            } = arm;

            let mut body_buffer = self.create_inner();
            body_buffer.push_block(body);
            body_buffer.flush_static_str();

            arms_buffer.size_hint = max(arms_buffer.size_hint, body_buffer.size_hint);
            arms_buffer.has_dynamic_value =
                arms_buffer.has_dynamic_value || body_buffer.has_dynamic_value;

            let body_token_stream = body_buffer.token_stream;

            match guard {
                None => arms_buffer.token_stream.extend(quote! {
                    #pattern => {
                        #body_token_stream
                    }
                }),
                Some(guard) => arms_buffer.token_stream.extend(quote! {
                    #pattern if #guard => {
                        #body_token_stream
                    }
                }),
            }
        }

        arms_buffer.flush_static_str();

        let arms_token_stream = arms_buffer.token_stream;

        self.token_stream.extend(quote! {
            match #expression {
                #arms_token_stream
            }
        });

        self.size_hint += arms_buffer.size_hint;
        self.has_dynamic_value = self.has_dynamic_value || arms_buffer.has_dynamic_value;
    }

    fn push_for_loop(&mut self, for_loop: &ForLoop) {
        self.flush_static_str();

        let ForLoop {
            pattern,
            expression,
            body,
        } = for_loop;

        let mut body_buffer = self.create_inner();
        body_buffer.push_block(body);
        body_buffer.flush_static_str();

        let body_token_stream = body_buffer.token_stream;

        self.token_stream.extend(quote! {
            for #pattern in #expression {
                #body_token_stream
            }
        });

        self.has_dynamic_value = true;
        self.size_hint += body_buffer.size_hint;
    }

    fn push_element(&mut self, element: &Element) {
        let Element {
            tag,
            attributes,
            children,
        } = element;

        self.static_str.push_str(&format!("<{}", tag.value()));

        for attribute in attributes {
            self.push_attribute(attribute);
        }

        self.static_str.push_str(">");

        if !is_void_element(&tag.value()) {
            self.push_block(children);
            self.static_str.push_str(&format!("</{}>", tag.value()));
        }
    }

    fn push_children(&mut self, children: &Ident) {
        self.flush_static_str();

        let writer = &self.writer;

        self.token_stream.extend(quote! {
            #children(#writer)?;
        });
    }

    fn push_component_call(&mut self, component_call: &ComponentCall) {
        self.flush_static_str();

        let ComponentCall {
            path,
            fields,
            attributes,
            children,
        } = component_call;

        let mut field_statements = Vec::with_capacity(fields.len());

        for field in fields {
            let ident = &field.ident;
            let value = &field.value;

            field_statements.push(quote! {
                #ident : #value
            });
        }

        let component_statement = quote! {
            &#path {
                #(#field_statements),*
            }
        };

        let mut attributes_buffer = self.create_inner();
        for attribute in attributes {
            attributes_buffer.push_attribute(attribute);
        }
        attributes_buffer.flush_static_str();

        let attributes_token_stream = attributes_buffer.token_stream;

        let mut children_buffer = self.create_inner();
        children_buffer.push_block(children);
        children_buffer.flush_static_str();

        let children_token_stream = children_buffer.token_stream;

        self.size_hint += attributes_buffer.size_hint + children_buffer.size_hint;
        self.has_dynamic_value = true;

        let writer = &self.writer;

        self.token_stream.extend(quote! {
            ::plait::Component::render_component(
                #component_statement,
                #writer,
                |#writer: &mut (dyn ::core::fmt::Write + '_)| -> ::core::fmt::Result {
                    #attributes_token_stream
                    Ok(())
                },
                |#writer: &mut (dyn ::core::fmt::Write + '_)| -> ::core::fmt::Result {
                    #children_token_stream
                    Ok(())
                },
            )?;
        });
    }

    fn push_dynamic_expr_escaped(&mut self, expr: &Expr) {
        self.flush_static_str();

        let writer = &self.writer;
        self.token_stream.extend(quote! {
            ::plait::RenderEscaped::render_escaped(&#expr, #writer)?;
        });

        self.has_dynamic_value = true;
    }

    fn push_dynamic_expr_raw(&mut self, expr: &Expr) {
        self.flush_static_str();

        let writer = &self.writer;
        self.token_stream.extend(quote! {
            ::plait::RenderRaw::render_raw(&#expr, #writer)?;
        });

        self.has_dynamic_value = true;
    }

    fn push_attribute(&mut self, attribute: &Attribute) {
        match attribute {
            Attribute::Spread(attrs) => {
                self.flush_static_str();

                let writer = &self.writer;

                self.token_stream.extend(quote! {
                    #attrs(#writer)?;
                });
            }
            Attribute::NameValue(name_value_attribute) => {
                match (name_value_attribute.is_maybe, &name_value_attribute.value) {
                    (false, None) => {
                        self.static_str
                            .push_str(&format!(" {}", name_value_attribute.name.value()));
                    }
                    (false, Some(value)) => {
                        self.static_str
                            .push_str(&format!(" {}=\"", name_value_attribute.name.value()));

                        match value {
                            AttributeValue::LitStr(lit_str) => self.push_lit_str_escaped(lit_str),
                            AttributeValue::LitChar(lit_char) => {
                                self.push_lit_char_escaped(lit_char)
                            }
                            AttributeValue::LitInt(lit_int) => self.push_lit_int(lit_int),
                            AttributeValue::LitFloat(lit_float) => self.push_lit_float(lit_float),
                            AttributeValue::LitBool(lit_bool) => self.push_lit_bool(lit_bool),
                            AttributeValue::Escaped(expr) => match &expr {
                                Expr::Lit(expr_lit) => match &expr_lit.lit {
                                    Lit::Str(lit_str) => self.push_lit_str_escaped(lit_str),
                                    Lit::Char(lit_char) => self.push_lit_char_escaped(lit_char),
                                    Lit::Int(lit_int) => self.push_lit_int(lit_int),
                                    Lit::Float(lit_float) => self.push_lit_float(lit_float),
                                    Lit::Bool(lit_bool) => self.push_lit_bool(lit_bool),
                                    _ => self.push_dynamic_expr_escaped(expr),
                                },
                                _ => self.push_dynamic_expr_escaped(expr),
                            },
                            AttributeValue::Raw(expr) => match &expr {
                                Expr::Lit(expr_lit) => match &expr_lit.lit {
                                    Lit::Str(lit_str) => self.push_lit_str_raw(lit_str),
                                    Lit::Char(lit_char) => self.push_lit_char_raw(lit_char),
                                    Lit::Int(lit_int) => self.push_lit_int(lit_int),
                                    Lit::Float(lit_float) => self.push_lit_float(lit_float),
                                    Lit::Bool(lit_bool) => self.push_lit_bool(lit_bool),
                                    _ => self.push_dynamic_expr_raw(expr),
                                },
                                _ => self.push_dynamic_expr_raw(expr),
                            },
                        }

                        self.static_str.push_str("\"");
                    }
                    (true, None) => {}
                    (true, Some(value)) => match value {
                        AttributeValue::LitStr(lit_str) => {
                            self.static_str
                                .push_str(&format!(" {}=\"", name_value_attribute.name.value()));
                            self.push_lit_str_escaped(lit_str);
                            self.static_str.push_str("\"");
                        }
                        AttributeValue::LitChar(lit_char) => {
                            self.static_str
                                .push_str(&format!(" {}=\"", name_value_attribute.name.value()));
                            self.push_lit_char_escaped(lit_char);
                            self.static_str.push_str("\"");
                        }
                        AttributeValue::LitInt(lit_int) => {
                            self.static_str
                                .push_str(&format!(" {}=\"", name_value_attribute.name.value()));
                            self.push_lit_int(lit_int);
                            self.static_str.push_str("\"");
                        }
                        AttributeValue::LitFloat(lit_float) => {
                            self.static_str
                                .push_str(&format!(" {}=\"", name_value_attribute.name.value()));
                            self.push_lit_float(lit_float);
                            self.static_str.push_str("\"");
                        }
                        AttributeValue::LitBool(lit_bool) => {
                            if lit_bool.value {
                                self.static_str
                                    .push_str(&format!(" {}", name_value_attribute.name.value()));
                            }
                        }
                        AttributeValue::Escaped(expr) => match &expr {
                            Expr::Lit(expr_lit) => match &expr_lit.lit {
                                Lit::Str(lit_str) => {
                                    self.static_str.push_str(&format!(
                                        " {}=\"",
                                        name_value_attribute.name.value()
                                    ));
                                    self.push_lit_str_escaped(lit_str);
                                    self.static_str.push_str("\"");
                                }
                                Lit::Char(lit_char) => {
                                    self.static_str.push_str(&format!(
                                        " {}=\"",
                                        name_value_attribute.name.value()
                                    ));
                                    self.push_lit_char_escaped(lit_char);
                                    self.static_str.push_str("\"");
                                }
                                Lit::Int(lit_int) => {
                                    self.static_str.push_str(&format!(
                                        " {}=\"",
                                        name_value_attribute.name.value()
                                    ));
                                    self.push_lit_int(lit_int);
                                    self.static_str.push_str("\"");
                                }
                                Lit::Float(lit_float) => {
                                    self.static_str.push_str(&format!(
                                        " {}=\"",
                                        name_value_attribute.name.value()
                                    ));
                                    self.push_lit_float(lit_float);
                                    self.static_str.push_str("\"");
                                }
                                Lit::Bool(lit_bool) => {
                                    if lit_bool.value {
                                        self.static_str.push_str(&format!(
                                            " {}",
                                            name_value_attribute.name.value()
                                        ));
                                    }
                                }
                                _ => {
                                    self.flush_static_str();

                                    let writer = &self.writer;
                                    let name = &name_value_attribute.name;

                                    self.token_stream.extend(quote! {
                                        ::plait::RenderMaybeAttributeEscaped::render_maybe_attribute_escaped(&#expr, #name, #writer)?;
                                    });

                                    self.has_dynamic_value = true;
                                }
                            },
                            _ => {
                                self.flush_static_str();

                                let writer = &self.writer;
                                let name = &name_value_attribute.name;

                                self.token_stream.extend(quote! {
                                    ::plait::RenderMaybeAttributeEscaped::render_maybe_attribute_escaped(&#expr, #name, #writer)?;
                                });

                                self.has_dynamic_value = true;
                            }
                        },
                        AttributeValue::Raw(expr) => match &expr {
                            Expr::Lit(expr_lit) => match &expr_lit.lit {
                                Lit::Str(lit_str) => {
                                    self.static_str.push_str(&format!(
                                        " {}=\"{}\"",
                                        name_value_attribute.name.value(),
                                        lit_str.value()
                                    ));
                                }
                                Lit::Char(lit_char) => {
                                    self.static_str.push_str(&format!(
                                        " {}=\"{}\"",
                                        name_value_attribute.name.value(),
                                        lit_char.value()
                                    ));
                                }
                                Lit::Int(lit_int) => {
                                    self.static_str.push_str(&format!(
                                        " {}=\"{}\"",
                                        name_value_attribute.name.value(),
                                        lit_int.base10_digits()
                                    ));
                                }
                                Lit::Float(lit_float) => {
                                    self.static_str.push_str(&format!(
                                        " {}=\"{}\"",
                                        name_value_attribute.name.value(),
                                        lit_float.base10_digits()
                                    ));
                                }
                                Lit::Bool(lit_bool) => {
                                    if lit_bool.value {
                                        self.static_str.push_str(&format!(
                                            " {}",
                                            name_value_attribute.name.value()
                                        ));
                                    }
                                }
                                _ => {
                                    self.flush_static_str();

                                    let writer = &self.writer;
                                    let name = &name_value_attribute.name;

                                    self.token_stream.extend(quote! {
                                        ::plait::RenderMaybeAttributeRaw::render_maybe_attribute_raw(&#expr, #name, #writer)?;
                                    });

                                    self.has_dynamic_value = true;
                                }
                            },
                            _ => {
                                self.flush_static_str();

                                let writer = &self.writer;
                                let name = &name_value_attribute.name;

                                self.token_stream.extend(quote! {
                                    ::plait::RenderMaybeAttributeRaw::render_maybe_attribute_raw(&#expr, #name, #writer)?;
                                });

                                self.has_dynamic_value = true;
                            }
                        },
                    },
                }
            }
        }
    }

    pub fn flush_static_str(&mut self) {
        if self.static_str.is_empty() {
            return;
        }

        let ident = &self.writer;
        let static_str = &self.static_str;
        self.token_stream.extend(quote! {
            ::core::fmt::Write::write_str(#ident, #static_str)?;
        });
        self.size_hint += static_str.len();
        self.static_str.clear();
    }

    fn create_inner(&self) -> Self {
        Self::new(self.writer.clone())
    }
}
