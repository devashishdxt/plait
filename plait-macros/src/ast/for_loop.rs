use syn::{
    Expr, Pat, braced,
    parse::{Parse, ParseStream},
    token::{For, In},
};

use crate::ast::Node;

#[derive(Debug)]
pub struct ForLoop {
    pub pattern: Pat,
    pub expression: Expr,
    pub body: Vec<Node>,
}

impl Parse for ForLoop {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_for_loop(input)
    }
}

fn parse_for_loop(input: ParseStream<'_>) -> syn::Result<ForLoop> {
    let _: For = input.parse()?;
    let pattern = Pat::parse_multi_with_leading_vert(input)?;
    let _: In = input.parse()?;
    let expression = input.call(Expr::parse_without_eager_brace)?;

    let content;
    let _ = braced!(content in input);

    let mut body = Vec::new();

    while !content.is_empty() {
        body.push(content.parse()?);
    }

    Ok(ForLoop {
        pattern,
        expression,
        body,
    })
}
