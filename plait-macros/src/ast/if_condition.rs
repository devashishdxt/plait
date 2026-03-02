use syn::Expr;

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
