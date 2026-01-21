use syn::{
    Expr, Pat, braced,
    parse::{Parse, ParseStream},
    token::{Comma, FatArrow, If, Match},
};

use crate::ast::Node;

/// A match expression in the template AST.
///
/// Represents pattern matching using `@match` syntax, allowing you to render
/// different content based on the structure of a value.
///
/// # Syntax
///
/// ```text
/// @match value {
///     Pattern => content,
///     Pattern if guard => content,
///     _ => fallback,
/// }
/// ```
#[derive(Debug)]
pub struct MatchExpression {
    /// The expression to match against.
    pub expression: Expr,

    /// The match arms.
    pub arms: Vec<MatchArm>,
}

impl Parse for MatchExpression {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_match_expression(input)
    }
}

/// A single arm in a match expression.
///
/// Each arm consists of a pattern, an optional guard condition, and the
/// content to render if the pattern matches.
#[derive(Debug)]
pub struct MatchArm {
    /// The pattern to match against.
    pub pattern: Pat,

    /// An optional guard condition: `if condition`.
    pub guard: Option<Expr>,

    /// The node to render if this arm matches.
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
