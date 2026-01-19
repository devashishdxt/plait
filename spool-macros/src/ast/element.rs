use syn::{
    Ident, LitStr,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    token::{Brace, Minus, Semi},
};

use crate::ast::{Attribute, Node};

/// An element in the HTML.
#[derive(Debug)]
pub struct Element {
    pub name: LitStr,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

impl Parse for Element {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_element(input)
    }
}

fn parse_element(input: ParseStream<'_>) -> syn::Result<Element> {
    let (name, is_void) = parse_element_name(input)?;
    let attributes = parse_element_attributes(input)?;

    if is_void {
        if !input.peek(Semi) {
            return Err(input.error("expected a `;` after a void element"));
        }
        input.parse::<Semi>()?;

        Ok(Element {
            name,
            attributes,
            children: Vec::new(),
        })
    } else if input.peek(Brace) {
        let children = parse_element_children(input)?;

        Ok(Element {
            name,
            attributes,
            children,
        })
    } else {
        Err(input.error("expected a body of the element enclosed in `{}`"))
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

fn parse_element_name(input: ParseStream<'_>) -> syn::Result<(LitStr, bool)> {
    if !input.peek(Ident::peek_any) {
        return Err(input.error("expected element name"));
    }

    let first_ident: Ident = input.call(Ident::parse_any)?;
    let first_span = first_ident.span();
    let mut last_span = first_span;
    let mut name = first_ident.to_string().to_lowercase();

    // Parse hyphenated segments: custom-element, my-custom-component, etc.
    while input.peek(Minus) {
        input.parse::<Minus>()?;

        if !input.peek(Ident::peek_any) {
            return Err(input.error("element name cannot end with hyphen"));
        }

        let next_ident: Ident = input.call(Ident::parse_any)?;
        last_span = next_ident.span();
        name.push('-');
        name.push_str(&next_ident.to_string().to_lowercase());
    }

    let span = first_span.join(last_span).unwrap_or(first_span);
    Ok((LitStr::new(&name, span), is_void_element(&name)))
}

fn parse_element_attributes(input: ParseStream<'_>) -> syn::Result<Vec<Attribute>> {
    let mut attributes = Vec::new();

    while !input.peek(Semi) && !input.peek(Brace) {
        attributes.push(input.parse()?);
    }

    Ok(attributes)
}

fn parse_element_children(input: ParseStream<'_>) -> syn::Result<Vec<Node>> {
    let content;
    syn::braced!(content in input);

    let mut children = Vec::new();
    while !content.is_empty() {
        children.push(content.parse()?);
    }

    Ok(children)
}
