use syn::{Expr, Pat};

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
