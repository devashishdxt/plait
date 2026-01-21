mod attribute_name;
mod attribute_value;

pub use self::{attribute_name::AttributeName, attribute_value::AttributeValue};

use syn::{
    Expr, parenthesized,
    parse::{Parse, ParseStream},
    token::{Dot, Eq, Paren, Question},
};

/// An HTML attribute in the template AST.
///
/// Attributes can be either name-value pairs with various value types, or spread
/// patterns that merge in attributes from another collection.
///
/// # Syntax
///
/// ```text
/// // Name-value pairs
/// name="literal"           // Literal string
/// name=(expr)              // Dynamic expression
/// name=[optional_expr]     // Optional (renders if Some)
/// name?[bool_expr]         // Boolean (renders name only if true)
/// name                     // Boolean (always true)
///
/// // Spread pattern
/// ..(attrs_expr)           // Merge attributes from expression
/// ```
#[derive(Debug)]
pub enum Attribute {
    /// A name-value pair attribute.
    ///
    /// If `value` is `None`, the attribute is rendered as a boolean attribute
    /// that is always present (e.g., `disabled`).
    NameValue {
        /// The attribute name.
        name: AttributeName,
        /// The attribute value, or `None` for boolean attributes without a value.
        value: Option<AttributeValue>,
    },

    /// A spread pattern that merges attributes from an expression.
    ///
    /// The expression must evaluate to an [`Attributes`](plait::Attributes) value.
    Spread {
        /// The expression providing the attributes to spread.
        expr: Expr,
    },
}

impl Parse for Attribute {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_attribute(input)
    }
}

fn parse_attribute(input: ParseStream<'_>) -> syn::Result<Attribute> {
    // Check for spread pattern: ..(expr)
    if input.peek(Dot) && input.peek2(Dot) {
        let _dot1: Dot = input.parse()?;
        let _dot2: Dot = input.parse()?;

        if !input.peek(Paren) {
            return Err(input.error("expected '(expression)' after '..'"));
        }

        let content;
        let _: Paren = parenthesized!(content in input);
        let expr: Expr = content.parse()?;

        return Ok(Attribute::Spread { expr });
    }

    // Otherwise, parse as name-value pair
    let name: AttributeName = input.parse()?;

    let value: Option<AttributeValue> = if input.peek(Question) || input.peek(Eq) {
        Some(input.parse()?)
    } else {
        None
    };

    Ok(Attribute::NameValue { name, value })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> syn::Result<Attribute> {
        syn::parse_str(input)
    }

    // Name-value pair tests

    #[test]
    fn name_value_literal() {
        let result = parse(r#"class="container""#).unwrap();
        match result {
            Attribute::NameValue { name, value, .. } => {
                assert_eq!(name.name.value(), "class");
                assert!(matches!(value, Some(AttributeValue::Literal { .. })));
            }
            _ => panic!("expected NameValue"),
        }
    }

    #[test]
    fn name_value_dynamic() {
        let result = parse("href=(url)").unwrap();
        match result {
            Attribute::NameValue { name, value, .. } => {
                assert_eq!(name.name.value(), "href");
                assert!(matches!(value, Some(AttributeValue::Dynamic { .. })));
            }
            _ => panic!("expected NameValue"),
        }
    }

    #[test]
    fn name_value_optional() {
        let result = parse("alt=[maybe_alt]").unwrap();
        match result {
            Attribute::NameValue { name, value, .. } => {
                assert_eq!(name.name.value(), "alt");
                assert!(matches!(value, Some(AttributeValue::Optional { .. })));
            }
            _ => panic!("expected NameValue"),
        }
    }

    #[test]
    fn name_value_boolean() {
        let result = parse("checked?[is_checked]").unwrap();
        match result {
            Attribute::NameValue { name, value, .. } => {
                assert_eq!(name.name.value(), "checked");
                assert!(matches!(value, Some(AttributeValue::Boolean { .. })));
            }
            _ => panic!("expected NameValue"),
        }
    }

    #[test]
    fn name_value_complex_name() {
        let result = parse(r#"hx-on:click="handler""#).unwrap();
        match result {
            Attribute::NameValue { name, value, .. } => {
                assert_eq!(name.name.value(), "hx-on:click");
                assert!(matches!(value, Some(AttributeValue::Literal { .. })));
            }
            _ => panic!("expected NameValue"),
        }
    }

    #[test]
    fn name_value_at_prefixed() {
        let result = parse("@click=(handle_click)").unwrap();
        match result {
            Attribute::NameValue { name, value, .. } => {
                assert_eq!(name.name.value(), "@click");
                assert!(matches!(value, Some(AttributeValue::Dynamic { .. })));
            }
            _ => panic!("expected NameValue"),
        }
    }

    // Spread pattern tests

    #[test]
    fn spread_simple() {
        let result = parse("..(attrs)").unwrap();
        match result {
            Attribute::Spread { .. } => {}
            _ => panic!("expected Spread"),
        }
    }

    #[test]
    fn spread_complex_expression() {
        let result = parse("..(get_attrs())").unwrap();
        match result {
            Attribute::Spread { .. } => {}
            _ => panic!("expected Spread"),
        }
    }

    #[test]
    fn spread_field_access() {
        let result = parse("..(self.attrs)").unwrap();
        match result {
            Attribute::Spread { .. } => {}
            _ => panic!("expected Spread"),
        }
    }

    #[test]
    fn spread_method_chain() {
        let result = parse("..(props.extra_attrs())").unwrap();
        match result {
            Attribute::Spread { .. } => {}
            _ => panic!("expected Spread"),
        }
    }

    // Error tests

    #[test]
    fn error_spread_missing_parens() {
        let result = parse("..attrs");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected '(expression)' after '..'")
        );
    }

    #[test]
    fn error_empty_input() {
        let result = parse("");
        assert!(result.is_err());
    }
}
