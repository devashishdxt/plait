use syn::{
    Attribute, Generics, Ident, Type, Visibility, braced, parenthesized,
    parse::{Parse, ParseStream},
    token::{Colon, Comma, Fn, Paren},
};

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

impl Parse for ComponentDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let visibility = input.parse()?;
        let _ = input.parse::<Fn>()?;
        let ident = input.parse()?;
        let mut generics = input.parse::<Generics>()?;

        let fields = if input.peek(Paren) {
            let content;
            let _ = parenthesized!(content in input);

            let mut fields = Vec::new();

            while !content.is_empty() {
                fields.push(content.parse()?);

                if content.peek(Comma) {
                    let _ = content.parse::<Comma>()?;
                }
            }

            fields
        } else {
            Vec::new()
        };

        generics.where_clause = input.parse()?;

        let content;
        let _ = braced!(content in input);

        let mut body = Vec::new();

        while !content.is_empty() {
            body.push(content.parse()?);
        }

        Ok(Self {
            attributes,
            visibility,
            ident,
            generics,
            fields,
            body,
        })
    }
}

impl Parse for ComponentDefinitionField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let _ = input.parse::<Colon>()?;
        let ty = input.parse()?;
        Ok(Self { ident, ty })
    }
}
