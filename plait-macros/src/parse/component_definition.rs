use syn::{
    Attribute, Generics, braced, parenthesized,
    parse::{Parse, ParseStream},
    token::{Colon, Comma, Eq, Fn, Paren},
};

use crate::ast::{ComponentDefinition, ComponentDefinitionField};

impl Parse for ComponentDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = Vec::new();
        let mut reserved_attrs = Vec::new();
        for attribute in input.call(Attribute::parse_outer)? {
            if attribute.path().is_ident("reserve_attrs") {
                let names = attribute.parse_args_with(|input: ParseStream| {
                    syn::punctuated::Punctuated::<syn::LitStr, Comma>::parse_terminated_with(
                        input,
                        super::attribute::parse_name,
                    )
                })?;
                reserved_attrs.extend(names);
            } else {
                attributes.push(attribute);
            }
        }
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
            reserved_attrs,
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
            input.parse::<Eq>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self { ident, ty, default })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn defaults_preserve_expressions_and_nested_bounds() {
        let definition: ComponentDefinition = syn::parse2(quote! {
            fn Example(label: &str = "hello", body: Option<impl PartialHtml> = None::<Fragment>, required: bool) {}
        }).unwrap();
        let default = &definition.fields[0].default;
        assert_eq!(quote!(#default).to_string(), "\"hello\"");
        let default = &definition.fields[1].default;
        assert_eq!(quote!(#default).to_string(), "None :: < Fragment >");
        let ty = &definition.fields[1].ty;
        assert_eq!(quote!(#ty).to_string(), "Option < impl PartialHtml >");
        assert!(definition.fields[2].default.is_none());
    }

    #[test]
    fn reservation_names_and_unrelated_attributes() {
        let definition: ComponentDefinition = syn::parse2(quote! {
            /// retained docs
            #[derive(Debug)]
            #[reserve_attrs(type, class, aria_disabled, "x-on:click", "TYPE",)]
            #[reserve_attrs()]
            #[reserve_attrs(class)]
            fn Example() {}
        })
        .unwrap();
        assert_eq!(definition.attributes.len(), 2);
        let names: Vec<_> = definition
            .reserved_attrs
            .iter()
            .map(syn::LitStr::value)
            .collect();
        assert_eq!(
            names,
            [
                "type",
                "class",
                "aria-disabled",
                "x-on:click",
                "TYPE",
                "class"
            ]
        );
        for (reservation, forwarded) in [
            ("aria-disabled", quote!(aria_disabled)),
            ("aria-disabled", quote!("aria-disabled")),
            ("type", quote!("TYPE")),
            ("type", quote!(type)),
            ("x-on:click", quote!("x-on:click")),
        ] {
            let attr: crate::ast::NameValueAttribute = syn::parse2(forwarded).unwrap();
            assert!(attr.name.value().eq_ignore_ascii_case(reservation));
        }
        let attr: crate::ast::NameValueAttribute = syn::parse2(quote!("TYPE")).unwrap();
        assert_eq!(attr.name.value(), "TYPE");
    }

    #[test]
    fn invalid_reservations_fail() {
        for annotation in [
            quote!(#[reserve_attrs(123)]),
            quote!(#[reserve_attrs(class = "x")]),
            quote!(#[reserve_attrs(foo::bar)]),
            quote!(#[reserve_attrs(type class)]),
            quote!(#[reserve_attrs]),
            quote!(#[reserve_attrs = "type"]),
        ] {
            assert!(syn::parse2::<ComponentDefinition>(quote!(#annotation fn Bad() {})).is_err());
        }
    }

    #[test]
    fn malformed_defaults_fail() {
        for input in [
            quote!(fn Bad(x: bool =) {}),
            quote!(fn Bad(x: bool = , y: bool) {}),
        ] {
            assert!(syn::parse2::<ComponentDefinition>(input).is_err());
        }
    }
}
