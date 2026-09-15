use std::collections::HashSet;

use syn::{
    Ident, braced,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    token::{At, Colon, Comma, Paren, Semi},
};

use crate::ast::{ComponentCall, ComponentCallField};

impl Parse for ComponentCall {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let _ = input.parse::<At>()?;
        let path = input.parse()?;

        let (fields, attributes) = if input.peek(Paren) {
            let content;
            let _ = parenthesized!(content in input);

            let mut fields = Vec::new();
            let mut names = HashSet::new();
            let mut attributes = Vec::new();

            if content.peek(Semi) {
                let _ = content.parse::<Semi>()?;
            } else {
                while !content.is_empty() {
                    let field: ComponentCallField = content.parse()?;
                    if !names.insert(field.ident.unraw().to_string()) {
                        return Err(syn::Error::new(
                            field.ident.span(),
                            "duplicate component prop",
                        ));
                    }
                    fields.push(field);

                    if content.peek(Comma) {
                        let _ = content.parse::<Comma>()?;
                    } else if content.peek(Semi) {
                        let _ = content.parse::<Semi>()?;
                        break;
                    } else if !content.is_empty() {
                        return Err(content.error("expected ',' or ';' after a field"));
                    }
                }
            }

            while !content.is_empty() {
                attributes.push(content.parse()?);

                if content.peek(Comma) {
                    let _ = content.parse::<Comma>()?;
                } else if !content.is_empty() {
                    return Err(content.error("expected ',' after an attribute"));
                }
            }

            (fields, attributes)
        } else {
            (vec![], vec![])
        };

        let content;
        let _ = braced!(content in input);

        let mut children = Vec::new();

        while !content.is_empty() {
            children.push(content.parse()?);
        }

        Ok(Self {
            path,
            fields,
            attributes,
            children,
        })
    }
}

impl Parse for ComponentCallField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        let value = if input.peek(Colon) {
            let _ = input.parse::<Colon>()?;
            input.parse().map(Some)
        } else {
            Ok(None)
        }?;

        Ok(Self { ident, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn duplicate_props_are_rejected() {
        for input in [
            quote!(@C(x: 1, x: 2) {}),
            quote!(@C(x, x: 2) {}),
            quote!(@C(x, x) {}),
            quote!(@C(x: 1, r#x) {}),
        ] {
            let error = syn::parse2::<ComponentCall>(input).err().unwrap();
            assert_eq!(error.to_string(), "duplicate component prop");
        }
        assert!(syn::parse2::<ComponentCall>(quote!(@C(x, y: 2; x: "attr") {})).is_ok());
    }
}
