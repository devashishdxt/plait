use syn::{
    LitStr,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
};

use crate::ast::{Element, ParenthesizedExpression};

#[derive(Debug)]
pub enum Node {
    Text(LitStr),
    Expression(ParenthesizedExpression),
    Element(Element),
    Fragment(Vec<Node>),
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
