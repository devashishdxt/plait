use syn::{Attribute, Generics, Ident, Type, Visibility};

use crate::ast::Node;

pub struct ComponentDefinition {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: Vec<ComponentDefinitionField>,
    pub body: Vec<Node>,
}

pub struct ComponentDefinitionField {
    pub ident: Ident,
    pub ty: Type,
}
