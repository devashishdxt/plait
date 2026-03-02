use syn::LitStr;

use crate::ast::{Attribute, Node};

pub struct Element {
    pub tag: LitStr,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}
