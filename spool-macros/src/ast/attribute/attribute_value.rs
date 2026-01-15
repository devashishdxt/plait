use syn::{
    Expr, LitStr, bracketed,
    parse::{Parse, ParseStream},
    token::{Bracket, Eq, Paren, Question},
};

use crate::ast::{BracketedExpression, ParenthesizedExpression};

/// The value of an HTML attribute.
#[derive(Debug)]
pub enum AttributeValue {
    /// Literal attribute: `class="container"`
    Literal { value: LitStr },

    /// Dynamic attribute: `href=(url)`
    Dynamic { expr: ParenthesizedExpression },

    /// Optional attribute: `alt=[maybe_alt]` - only renders if Some
    Optional { expr: BracketedExpression },

    /// Boolean attribute: `checked?[is_checked]` - renders name only if true
    Boolean { expr: Expr },
}

impl Parse for AttributeValue {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_attribute_value(input)
    }
}

fn parse_attribute_value(input: ParseStream<'_>) -> syn::Result<AttributeValue> {
    // Check for boolean attribute: ?[expr]
    if input.peek(Question) {
        let _: Question = input.parse()?;
        let content;
        let _: Bracket = bracketed!(content in input);
        let expr: Expr = content.parse()?;
        return Ok(AttributeValue::Boolean { expr });
    }

    // All other variants start with =
    if !input.peek(Eq) {
        return Err(input.error("expected '=' or '?[bool_expr]' for attribute value"));
    }
    let _: Eq = input.parse()?;

    // Check what follows the =
    if input.peek(Paren) {
        // Dynamic: =(expr)
        let expr: ParenthesizedExpression = input.parse()?;
        Ok(AttributeValue::Dynamic { expr })
    } else if input.peek(Bracket) {
        // Optional: =[expr]
        let expr: BracketedExpression = input.parse()?;
        Ok(AttributeValue::Optional { expr })
    } else if input.peek(LitStr) {
        // Literal: ="value"
        let value: LitStr = input.parse()?;
        Ok(AttributeValue::Literal { value })
    } else {
        Err(input.error("expected string literal, (expression), or [expression] after '='"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> syn::Result<AttributeValue> {
        syn::parse_str(input)
    }

    #[test]
    fn literal_string() {
        let result = parse(r#"="hello""#).unwrap();
        assert!(matches!(result, AttributeValue::Literal { value } if value.value() == "hello"));

        let result = parse(r#"="container flex""#).unwrap();
        assert!(
            matches!(result, AttributeValue::Literal { value } if value.value() == "container flex")
        );
    }

    #[test]
    fn dynamic_expression() {
        let result = parse("=(url)").unwrap();
        assert!(matches!(result, AttributeValue::Dynamic { .. }));

        let result = parse("=(foo.bar())").unwrap();
        assert!(matches!(result, AttributeValue::Dynamic { .. }));

        let result = parse("=(a + b)").unwrap();
        assert!(matches!(result, AttributeValue::Dynamic { .. }));
    }

    #[test]
    fn optional_expression() {
        let result = parse("=[maybe_alt]").unwrap();
        assert!(matches!(result, AttributeValue::Optional { .. }));

        let result = parse("=[some_option.as_ref()]").unwrap();
        assert!(matches!(result, AttributeValue::Optional { .. }));
    }

    #[test]
    fn boolean_expression() {
        let result = parse("?[is_checked]").unwrap();
        assert!(matches!(result, AttributeValue::Boolean { .. }));

        let result = parse("?[a && b]").unwrap();
        assert!(matches!(result, AttributeValue::Boolean { .. }));

        let result = parse("?[items.is_empty()]").unwrap();
        assert!(matches!(result, AttributeValue::Boolean { .. }));
    }

    #[test]
    fn error_missing_equals_or_question() {
        let result = parse("hello");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected '=' or '?[bool_expr]'")
        );
    }

    #[test]
    fn error_invalid_after_equals() {
        let result = parse("= invalid");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected string literal, (expression), or [expression] after '='")
        );
    }
}
