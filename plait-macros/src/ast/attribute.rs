use convert_case::{Boundary, Case, Casing};
use syn::{
    Expr, Ident, LitStr,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    token::{Colon, Comma, Paren, Pound, Question},
};

pub enum Attribute {
    Spread(Ident),
    NameValue(NameValueAttribute),
}

pub struct NameValueAttribute {
    pub name: LitStr,
    pub is_maybe: bool,
    pub is_url: bool,
    pub value: Option<AttributeValue>,
}

pub enum AttributeValue {
    Text(LitStr),
    RawExpression(Expr),
    Expression(Expr),
}

impl Parse for Attribute {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(Pound) {
            let _ = input.parse::<Pound>()?;
            let ident = input.parse::<Ident>()?;

            if ident == "attrs" {
                Ok(Self::Spread(ident))
            } else {
                Err(syn::Error::new(
                    ident.span(),
                    "Invalid attribute, expected `attrs` after `#`",
                ))
            }
        } else {
            Ok(Self::NameValue(input.parse()?))
        }
    }
}

impl Parse for NameValueAttribute {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name = if input.peek(LitStr) {
            input.parse()?
        } else {
            let name_ident = input.call(Ident::parse_any)?;
            let name_string = name_ident
                .to_string()
                .set_boundaries(&[Boundary::Underscore])
                .to_case(Case::Kebab);
            LitStr::new(&name_string, name_ident.span())
        };

        let is_url = is_url_attribute(&name.value());

        if input.is_empty() || input.peek(Comma) {
            return Ok(Self {
                name,
                is_maybe: false,
                is_url,
                value: None,
            });
        }

        let is_maybe = input.peek(Question);
        if is_maybe {
            let _ = input.parse::<Question>()?;
        }

        let _ = input.parse::<Colon>()?;

        let value = Some(input.parse()?);

        Ok(Self {
            name,
            is_maybe,
            is_url,
            value,
        })
    }
}

impl Parse for AttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Text(input.parse()?))
        } else if input.peek(Pound) {
            let pound = input.parse::<Pound>()?;
            if input.peek(Paren) {
                let content;
                let _ = parenthesized!(content in input);

                Ok(Self::RawExpression(content.parse()?))
            } else {
                Err(syn::Error::new(
                    pound.span,
                    "Expected '(' after '#' for raw attribute value",
                ))
            }
        } else {
            Ok(Self::Expression(input.parse()?))
        }
    }
}

/// Returns true if the attribute name is a URL attribute.
fn is_url_attribute(name: &str) -> bool {
    matches!(
        name,
        "href"
            | "src"
            | "action"
            | "formaction"
            | "poster"
            | "cite"
            | "data"
            | "profile"
            | "manifest"
            | "icon"
            | "background"
            | "xlink:href"
    )
}
