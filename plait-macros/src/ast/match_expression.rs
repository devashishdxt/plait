use syn::{
    Expr, Pat, braced,
    parse::{Parse, ParseStream},
    token::{Brace, Comma, FatArrow, If, Match},
};

use crate::ast::Node;

pub struct MatchExpression {
    pub expression: Expr,

    pub arms: Vec<MatchArm>,
}

pub struct MatchArm {
    pub pattern: Pat,

    pub guard: Option<Expr>,

    pub body: Vec<Node>,
}

impl Parse for MatchExpression {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let _: Match = input.parse()?;
        let expression = Expr::parse_without_eager_brace(input)?;

        let content;
        let _ = braced!(content in input);

        let mut arms = Vec::new();
        while !content.is_empty() {
            arms.push(content.parse()?);
        }

        Ok(Self { expression, arms })
    }
}

impl Parse for MatchArm {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let pattern = Pat::parse_multi_with_leading_vert(input)?;
        let guard = if input.peek(If) {
            let _ = input.parse::<If>()?;
            Some(input.parse()?)
        } else {
            None
        };

        let _: FatArrow = input.parse()?;

        let body = if input.peek(Brace) {
            let content;
            let _ = braced!(content in input);
            let mut body = Vec::new();
            while !content.is_empty() {
                body.push(content.parse()?);
            }
            body
        } else {
            let node = input.parse()?;
            vec![node]
        };

        if input.peek(Comma) {
            let _ = input.parse::<Comma>()?;
        }

        Ok(Self {
            pattern,
            guard,
            body,
        })
    }
}
