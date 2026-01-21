use syn::{
    Expr, Pat, braced,
    parse::{Parse, ParseStream},
    token::{Comma, FatArrow, If, Match},
};

use crate::ast::Node;

#[derive(Debug)]
pub struct MatchExpression {
    pub expression: Expr,
    pub arms: Vec<MatchArm>,
}

impl Parse for MatchExpression {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_match_expression(input)
    }
}

#[derive(Debug)]
pub struct MatchArm {
    pub pattern: Pat,
    pub guard: Option<Expr>,
    pub body: Node,
}

impl Parse for MatchArm {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_match_arm(input)
    }
}

fn parse_match_expression(input: ParseStream<'_>) -> syn::Result<MatchExpression> {
    let _: Match = input.parse()?;
    let expression = Expr::parse_without_eager_brace(input)?;

    let content;
    let _ = braced!(content in input);

    let mut arms = Vec::new();
    while !content.is_empty() {
        arms.push(content.parse()?);
    }

    Ok(MatchExpression { expression, arms })
}

fn parse_match_arm(input: ParseStream<'_>) -> syn::Result<MatchArm> {
    let pattern = Pat::parse_multi_with_leading_vert(input)?;
    let guard = if input.peek(If) {
        let _ = input.parse::<If>()?;
        Some(input.parse()?)
    } else {
        None
    };

    let _: FatArrow = input.parse()?;

    let body = input.parse()?;

    if input.peek(Comma) {
        let _ = input.parse::<Comma>()?;
    }

    Ok(MatchArm {
        pattern,
        guard,
        body,
    })
}
