use syn::{Expr, Pat};

use crate::ast::Node;

pub struct ForLoop {
    pub pattern: Pat,
    pub expression: Expr,
    pub body: Vec<Node>,
}
