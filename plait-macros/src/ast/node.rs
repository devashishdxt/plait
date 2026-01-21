use syn::{
    LitStr,
    parse::{Parse, ParseStream},
    token::{At, Brace, For, If, Match, Paren},
};

use crate::ast::{Element, ForLoop, IfCondition, MatchExpression, ParenthesizedExpression};

#[derive(Debug)]
pub enum Node {
    Text(LitStr),
    Expression(ParenthesizedExpression),
    Fragment(Vec<Node>),
    If(IfCondition),
    For(ForLoop),
    Match(MatchExpression),
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
