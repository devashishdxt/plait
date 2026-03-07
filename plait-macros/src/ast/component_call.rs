use syn::{Expr, Ident, Path};

use crate::ast::{Attribute, Node};

pub struct ComponentCall {
    pub path: Path,
    pub fields: Vec<ComponentCallField>,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

pub struct ComponentCallField {
    pub ident: Ident,
    pub value: Option<Expr>,
}
