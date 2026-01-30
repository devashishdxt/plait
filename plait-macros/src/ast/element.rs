use convert_case::{Boundary, Case, Casing};
use syn::{
    Ident, LitStr, braced,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Comma, Paren, Semi},
};

use crate::ast::{Attribute, Node};

pub struct Element {
    pub name: LitStr,

    pub is_void: bool,

    pub attributes: Vec<Attribute>,

    pub children: Vec<Node>,
}

impl Parse for Element {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name_ident = input.call(Ident::parse_any)?;
        let name_string = name_ident
            .to_string()
            .set_boundaries(&[Boundary::Underscore])
            .to_case(Case::Kebab);

        let name = LitStr::new(&name_string, name_ident.span());
        let is_void = is_void_element(&name_string);

        let attributes = if input.peek(Paren) {
            let content;
            parenthesized!(content in input);

            let mut attributes = Vec::new();

            while !content.is_empty() {
                attributes.push(content.parse()?);

                if content.peek(Comma) {
                    let _ = content.parse::<Comma>()?;
                }
            }

            attributes
        } else {
            Vec::new()
        };

        if is_void {
            if !input.peek(Semi) {
                return Err(syn::Error::new(
                    name_ident.span(),
                    "expected a `;` after a void element",
                ));
            }
            input.parse::<Semi>()?;

            Ok(Self {
                name,
                is_void,
                attributes,
                children: Vec::new(),
            })
        } else if input.peek(Brace) {
            let content;
            braced!(content in input);

            let mut children = Vec::new();
            while !content.is_empty() {
                children.push(content.parse()?);
            }

            Ok(Self {
                name,
                is_void,
                attributes,
                children,
            })
        } else {
            Err(syn::Error::new(
                name_ident.span(),
                "expected a body of the element enclosed in `{}`",
            ))
        }
    }
}

/// Returns true if the given element name is a void element.
/// Expects the name to be in ASCII lowercase.
fn is_void_element(name: &str) -> bool {
    matches!(
        name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}
