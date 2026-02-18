use syn::{
    Expr, Ident, LitStr, parenthesized,
    parse::{Parse, ParseStream},
    token::{At, For, If, Let, Match, Paren, Pound},
};

use crate::ast::{ComponentCall, Element, ForLoop, IfCondition, LetBinding, MatchExpression};

pub enum Node {
    Text(LitStr),
    Expression(Expr),
    RawExpression(Expr),
    Doctype(Ident),
    Children(Ident),
    LetBinding(LetBinding),
    IfCondition(IfCondition),
    ForLoop(ForLoop),
    MatchExpression(MatchExpression),
    ComponentCall(ComponentCall),
    HtmlDisplay(Expr),
    Element(Element),
}

impl Parse for Node {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Text(input.parse()?))
        } else if input.peek(Paren) {
            let content;
            parenthesized!(content in input);

            Ok(Self::Expression(content.parse()?))
        } else if input.peek(Pound) {
            let _: Pound = input.parse()?;

            if input.peek(Paren) {
                let content;
                parenthesized!(content in input);

                Ok(Self::RawExpression(content.parse()?))
            } else if input.peek(Ident) {
                let ident: Ident = input.parse()?;

                if ident == "doctype" {
                    Ok(Self::Doctype(ident))
                } else if ident == "children" {
                    Ok(Self::Children(ident))
                } else {
                    Err(syn::Error::new(
                        ident.span(),
                        "Expected 'doctype'm 'children' or '(' after '#'",
                    ))
                }
            } else {
                Err(syn::Error::new(
                    input.span(),
                    "Expected 'doctype' or '(' after '#'",
                ))
            }
        } else if input.peek(Let) {
            Ok(Self::LetBinding(input.parse()?))
        } else if input.peek(If) {
            Ok(Self::IfCondition(input.parse()?))
        } else if input.peek(For) {
            Ok(Self::ForLoop(input.parse()?))
        } else if input.peek(Match) {
            Ok(Self::MatchExpression(input.parse()?))
        } else if input.peek(At) {
            if input.peek2(Paren) {
                let _: At = input.parse()?;

                let content;
                parenthesized!(content in input);

                Ok(Self::HtmlDisplay(content.parse()?))
            } else {
                Ok(Self::ComponentCall(input.parse()?))
            }
        } else {
            Ok(Self::Element(input.parse()?))
        }
    }
}
