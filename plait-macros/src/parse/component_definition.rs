use syn::{
    Attribute, Generics, braced, parenthesized,
    parse::{Parse, ParseStream},
    token::{Colon, Comma, Eq, Fn, Paren},
};

use crate::ast::{ComponentDefinition, ComponentDefinitionField};

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
                } else if !content.is_empty() {
                    return Err(content.error("expected ',' or ')'"));
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
        let default = if input.peek(Eq) {
            let _ = input.parse::<Eq>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self { ident, ty, default })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ComponentDefinition;
    use quote::{ToTokens, quote};

    #[test]
    fn explicit_defaults_do_not_change_requiredness_of_other_props() {
        let component: ComponentDefinition = syn::parse2(quote! {
            pub fn Example(
                id: &str,
                required_option: Option<&str>,
                value: &str = "",
                optional: Option<&str> = None,
                pair: (u32, u32) = (1, 2),
            ) {}
        })
        .unwrap();
        assert!(component.fields[0].default.is_none());
        assert!(component.fields[1].default.is_none());
        assert_eq!(
            component.fields[2]
                .default
                .as_ref()
                .unwrap()
                .to_token_stream()
                .to_string(),
            "\"\""
        );
        assert_eq!(
            component.fields[3]
                .default
                .as_ref()
                .unwrap()
                .to_token_stream()
                .to_string(),
            "None"
        );
        assert_eq!(
            component.fields[4]
                .default
                .as_ref()
                .unwrap()
                .to_token_stream()
                .to_string(),
            "(1 , 2)"
        );
    }

    #[test]
    fn rejects_missing_default_expression() {
        assert!(
            syn::parse2::<ComponentDefinition>(quote! {
                fn Example(value: &str = ) {}
            })
            .is_err()
        );
    }
}
