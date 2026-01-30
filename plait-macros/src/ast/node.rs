use syn::{
    Expr, Ident, LitStr, parenthesized,
    parse::{Parse, ParseStream},
    token::{At, For, If, Match, Paren, Pound},
};

use crate::ast::{Element, ForLoop, IfCondition, MatchExpression, component_call::ComponentCall};

pub enum Node {
    Text(LitStr),
    RawExpression(Expr),
    Expression(Expr),
    Children(Ident),
    If(IfCondition),
    For(ForLoop),
    Match(MatchExpression),
    Component(ComponentCall),
    Element(Element),
}

impl Parse for Node {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Text(input.parse()?))
        } else if input.peek(Pound) {
            let pound = input.parse::<Pound>()?;

            if input.peek(Paren) {
                let content;
                let _ = parenthesized!(content in input);

                Ok(Self::RawExpression(content.parse()?))
            } else if input.peek(Ident) {
                let ident = input.parse::<Ident>()?;
                if ident == "children" {
                    Ok(Self::Children(ident))
                } else {
                    Err(syn::Error::new(
                        ident.span(),
                        "Expected `#children` for component children",
                    ))
                }
            } else {
                Err(syn::Error::new(
                    pound.span,
                    "Expected '(' after '#' for raw expression or `#children` for component children",
                ))
            }
        } else if input.peek(Paren) {
            let content;
            let _ = parenthesized!(content in input);

            Ok(Self::Expression(content.parse()?))
        } else if input.peek(If) {
            Ok(Self::If(input.parse()?))
        } else if input.peek(For) {
            Ok(Self::For(input.parse()?))
        } else if input.peek(Match) {
            Ok(Self::Match(input.parse()?))
        } else if input.peek(At) {
            Ok(Self::Component(input.parse()?))
        } else {
            Ok(Self::Element(input.parse()?))
        }
    }
}
