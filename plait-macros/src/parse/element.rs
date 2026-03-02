use convert_case::{Boundary, Case, Casing};
use syn::{
    Ident, LitStr, braced,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Comma, Paren, Semi},
};

use crate::{ast::Element, utils::is_void_element};

impl Parse for Element {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name_ident = input.call(Ident::parse_any)?;
        let name_string = name_ident
            .to_string()
            .set_boundaries(&[Boundary::Underscore])
            .to_case(Case::Kebab);

        let tag = LitStr::new(&name_string, name_ident.span());

        let is_void = is_void_element(&name_string);

        let attributes = if input.peek(Paren) {
            let content;
            parenthesized!(content in input);

            let mut attributes = Vec::new();

            while !content.is_empty() {
                attributes.push(content.parse()?);

                if content.peek(Comma) {
                    let _ = content.parse::<Comma>()?;
                } else if !content.is_empty() {
                    return Err(syn::Error::new(
                        content.span(),
                        "expected a `,` or `)` after an attribute",
                    ));
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
            let _: Semi = input.parse()?;

            Ok(Self {
                tag,
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
                tag,
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
