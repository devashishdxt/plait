use syn::{
    Ident, LitBool, LitChar, LitFloat, LitInt, LitStr, braced,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    token::{At, Brace, For, If, Let, Match, Paren, Pound},
};

use crate::ast::{Element, Node};

impl Parse for Node {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Node::LitStr(input.parse()?))
        } else if input.peek(LitChar) {
            Ok(Node::LitChar(input.parse()?))
        } else if input.peek(LitInt) {
            Ok(Node::LitInt(input.parse()?))
        } else if input.peek(LitFloat) {
            Ok(Node::LitFloat(input.parse()?))
        } else if input.peek(LitBool) {
            Ok(Node::LitBool(input.parse()?))
        } else if input.peek(Brace) {
            let content;
            braced!(content in input);

            let mut nodes = Vec::new();
            while !content.is_empty() {
                nodes.push(content.parse()?);
            }

            Ok(Node::Block(nodes))
        } else if input.peek(Let) {
            Ok(Node::LetBinding(input.parse()?))
        } else if input.peek(If) {
            Ok(Node::IfCondition(input.parse()?))
        } else if input.peek(Match) {
            Ok(Node::MatchExpression(input.parse()?))
        } else if input.peek(For) {
            Ok(Node::ForLoop(input.parse()?))
        } else if input.peek(Paren) {
            let content;
            parenthesized!(content in input);

            Ok(Node::Escaped(content.parse()?))
        } else if input.peek(Pound) {
            let _: Pound = input.parse()?;

            if input.peek(Paren) {
                let content;
                parenthesized!(content in input);

                Ok(Node::Raw(content.parse()?))
            } else if input.peek(Ident::peek_any) {
                let ident: Ident = input.parse()?;

                if ident == "doctype" {
                    Ok(Node::Doctype)
                } else if ident == "children" {
                    Ok(Node::Children(ident))
                } else {
                    Err(syn::Error::new(
                        ident.span(),
                        "unexpected identifier after `#`",
                    ))
                }
            } else {
                Err(input.error("unexpected token in html node"))
            }
        } else if input.peek(At) {
            Ok(Node::ComponentCall(input.parse()?))
        } else if input.peek(Ident::peek_any) {
            Ok(Node::Element(Element::parse(input)?))
        } else {
            Err(input.error("unexpected token in html node"))
        }
    }
}
