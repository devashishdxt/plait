use syn::{
    Expr, braced,
    parse::{Parse, ParseStream},
    token::{Else, If},
};

use crate::ast::Node;

pub struct IfCondition {
    pub condition: Expr,
    pub then_branch: Vec<Node>,
    pub else_branch: Option<ElseBranch>,
}

pub enum ElseBranch {
    If(Box<IfCondition>),
    Else(Vec<Node>),
}

impl Parse for IfCondition {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let _: If = input.parse()?;
        // Use parse_without_eager_brace to avoid parsing `condition {}` as a struct literal
        let condition = input.call(Expr::parse_without_eager_brace)?;

        let content;
        let _ = braced!(content in input);

        let mut then_branch = Vec::new();

        while !content.is_empty() {
            then_branch.push(content.parse()?);
        }

        let else_branch = if input.peek(Else) {
            let _: Else = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self {
            condition,
            then_branch,
            else_branch,
        })
    }
}

impl Parse for ElseBranch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(If) {
            Ok(Self::If(Box::new(input.parse()?)))
        } else {
            let content;
            let _ = braced!(content in input);

            let mut else_branch = Vec::new();

            while !content.is_empty() {
                else_branch.push(content.parse()?);
            }

            Ok(Self::Else(else_branch))
        }
    }
}
