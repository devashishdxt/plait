use syn::{
    Expr, parenthesized,
    parse::{Parse, ParseStream},
    token::Colon,
};

use crate::ast::EscapeMode;

/// A dynamic parenthesized expression that can be evaluated at runtime.
///
/// Format: `(expr : escape_mode)`. `escape_mode` is optional. If `escape_mode` is not provided, the default escape
/// mode will be resolved based on the context.
#[derive(Debug)]
pub struct ParenthesizedExpression {
    /// The expression to be evaluated.
    pub expr: Expr,

    /// The escape mode to use when rendering the output to HTML.
    pub escape_mode: Option<EscapeMode>,
}

impl Parse for ParenthesizedExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        parse_expression(input)
    }
}

fn parse_expression(input: ParseStream<'_>) -> syn::Result<ParenthesizedExpression> {
    let content;
    let _ = parenthesized!(content in input);

    let expr: Expr = content.parse()?;

    let escape_mode = if content.peek(Colon) {
        content.parse::<Colon>()?;
        Some(content.parse()?)
    } else {
        None
    };

    Ok(ParenthesizedExpression { expr, escape_mode })
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn test_parse_expression_without_escape_mode() {
        let input = quote! { (1 + 2) };
        let expression: ParenthesizedExpression = syn::parse2(input).unwrap();
        assert_eq!(expression.expr, syn::parse2(quote! { 1 + 2 }).unwrap());
        assert_eq!(expression.escape_mode, None);
    }

    #[test]
    fn test_parse_expression_with_escape_mode_raw() {
        let input = quote! { ("<div></div>" : raw) };
        let expression: ParenthesizedExpression = syn::parse2(input).unwrap();
        assert_eq!(
            expression.expr,
            syn::parse2(quote! { "<div></div>" }).unwrap()
        );
        assert_eq!(expression.escape_mode, Some(EscapeMode::Raw));
    }

    #[test]
    fn test_parse_expression_with_escape_mode_html() {
        let input = quote! { ("<div></div>" : html) };
        let expression: ParenthesizedExpression = syn::parse2(input).unwrap();
        assert_eq!(
            expression.expr,
            syn::parse2(quote! { "<div></div>" }).unwrap()
        );
        assert_eq!(expression.escape_mode, Some(EscapeMode::Html));
    }

    #[test]
    fn test_parse_expression_with_escape_mode_invalid() {
        let input = quote! { ("<div></div>" : invalid) };
        let result: syn::Result<ParenthesizedExpression> = syn::parse2(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_complex_expression_with_escape_mode_html() {
        let input = quote! { (self.process()? : html) };
        let expression: ParenthesizedExpression = syn::parse2(input).unwrap();
        assert_eq!(
            expression.expr,
            syn::parse2(quote! { self.process()? }).unwrap()
        );
        assert_eq!(expression.escape_mode, Some(EscapeMode::Html));
    }
}
