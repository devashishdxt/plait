use syn::{
    LitStr,
    parse::{Parse, ParseStream},
    token::{At, Brace, For, If, Match, Paren},
};

use crate::ast::{Element, ForLoop, IfCondition, MatchExpression, ParenthesizedExpression};

/// A node in the template AST representing a piece of content.
///
/// Nodes are the building blocks of templates and can represent static text,
/// dynamic expressions, HTML elements, or control flow constructs.
#[derive(Debug)]
pub enum Node {
    /// Literal text content: `"hello world"`.
    Text(LitStr),

    /// A dynamic expression: `(expr)` or `(expr : escape_mode)`.
    Expression(ParenthesizedExpression),

    /// A fragment containing multiple child nodes: `{ node1 node2 }`.
    Fragment(Vec<Node>),

    /// An if condition: `@if condition { ... }`.
    If(IfCondition),

    /// A for loop: `@for pattern in expr { ... }`.
    For(ForLoop),

    /// A match expression: `@match expr { ... }`.
    Match(MatchExpression),

    /// An HTML element: `div class="container" { ... }`.
    Element(Element),
}

impl Parse for Node {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_node(input)
    }
}

fn parse_node(input: ParseStream<'_>) -> syn::Result<Node> {
    if input.peek(LitStr) {
        Ok(Node::Text(input.parse()?))
    } else if input.peek(Paren) {
        Ok(Node::Expression(input.parse()?))
    } else if input.peek(Brace) {
        Ok(Node::Fragment(parse_fragment(input)?))
    } else if input.peek(At) {
        let _: At = input.parse()?;

        if input.peek(If) {
            Ok(Node::If(input.parse()?))
        } else if input.peek(For) {
            Ok(Node::For(input.parse()?))
        } else if input.peek(Match) {
            Ok(Node::Match(input.parse()?))
        } else {
            Err(input.error("unexpected control flow token"))
        }
    } else {
        Ok(Node::Element(input.parse()?))
    }
}

fn parse_fragment(input: ParseStream<'_>) -> syn::Result<Vec<Node>> {
    let content;
    syn::braced!(content in input);

    let mut children = Vec::new();
    while !content.is_empty() {
        children.push(content.parse()?);
    }

    Ok(children)
}
