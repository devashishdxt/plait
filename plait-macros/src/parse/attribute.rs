use convert_case::{Boundary, Case, Casing};
use syn::{
    Ident, LitBool, LitChar, LitFloat, LitInt, LitStr,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    token::{Colon, Comma, Paren, Pound, Question},
};

use crate::ast::{Attribute, AttributeValue, NameValueAttribute};

impl Parse for AttributeValue {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(AttributeValue::LitStr(input.parse()?))
        } else if input.peek(LitChar) {
            Ok(AttributeValue::LitChar(input.parse()?))
        } else if input.peek(LitInt) {
            Ok(AttributeValue::LitInt(input.parse()?))
        } else if input.peek(LitFloat) {
            Ok(AttributeValue::LitFloat(input.parse()?))
        } else if input.peek(LitBool) {
            Ok(AttributeValue::LitBool(input.parse()?))
        } else if input.peek(Paren) {
            let content;
            parenthesized!(content in input);

            Ok(AttributeValue::Escaped(content.parse()?))
        } else if input.peek(Pound) && input.peek2(Paren) {
            let _: Pound = input.parse()?;

            let content;
            parenthesized!(content in input);

            Ok(AttributeValue::Raw(content.parse()?))
        } else {
            Ok(AttributeValue::Escaped(input.parse()?))
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

        if input.is_empty() || input.peek(Comma) {
            return Ok(Self {
                name,
                is_maybe: false,
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
            value,
        })
    }
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
