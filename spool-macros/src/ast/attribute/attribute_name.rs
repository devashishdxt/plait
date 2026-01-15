use proc_macro2::Span;
use syn::{
    Ident,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    token::{At, Dot, Minus},
};

/// The name of an HTML attribute.
#[derive(Debug, Clone)]
pub struct AttributeName {
    pub name: String,
    pub span: Span,
}

impl Parse for AttributeName {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_attribute_name(input)
    }
}

/// Parses an HTML attribute name with practical restrictions for a templating library.
///
/// ## Restrictions
///
/// 1. **Starting character**: Must be ASCII letter, `@`, `_`, `$`, `:`, or `#`
/// 2. **Contains identifier**: Must contain at least one identifier (letters/digits/underscore)
/// 3. **No trailing punctuation**: Must end with an identifier, not `-`, `:`, or `.`
/// 4. **Allowed separators**: `-`, `:`, `.` between identifiers
///
/// ## Valid Examples
/// - `class`, `data-value`, `hx-get`
/// - `@click`, `@click.prevent`, `@keyup.page-down`
/// - `hx-on:click`, `xml:lang`
/// - `:class` (Vue shorthand)
/// - `#ref` (possible anchor ref)
/// - `_internal`, `$var`
///
/// ## Invalid Examples
/// - `@` (no identifier)
/// - `data-` (trailing punctuation)
/// - `123` (starts with number)
/// - `a&b` (disallowed punctuation)
fn parse_attribute_name(input: ParseStream<'_>) -> syn::Result<AttributeName> {
    use syn::token::{Colon, Dollar, Pound};

    let mut name = String::new();
    let mut first_span: Option<Span> = None;
    let mut last_span: Option<Span>;

    // Must have at least one token
    if input.is_empty() {
        return Err(input.error("expected attribute name"));
    }

    // Parse optional prefix: @, $, :, #
    if input.peek(At) {
        let token: At = input.parse()?;
        name.push('@');
        first_span = Some(token.span);
    } else if input.peek(Dollar) {
        let token: Dollar = input.parse()?;
        name.push('$');
        first_span = Some(token.span);
    } else if input.peek(Colon) {
        let token: Colon = input.parse()?;
        name.push(':');
        first_span = Some(token.span);
    } else if input.peek(Pound) {
        let token: Pound = input.parse()?;
        name.push('#');
        first_span = Some(token.span);
    }

    // Must have an identifier after optional prefix (or as first token)
    if !input.peek(Ident::peek_any) {
        if name.is_empty() {
            return Err(
                input.error("attribute name must start with a letter, '_', '@', '$', ':', or '#'")
            );
        } else {
            return Err(input.error(format!(
                "expected identifier after '{}'",
                name.chars().last().unwrap()
            )));
        }
    }

    // Parse first identifier
    let ident: Ident = input.call(Ident::parse_any)?;
    name.push_str(&ident.to_string());
    if first_span.is_none() {
        first_span = Some(ident.span());
    }
    last_span = Some(ident.span());

    // Continue parsing separator + identifier parts
    loop {
        if input.peek(Minus) {
            let _: Minus = input.parse()?;
            name.push('-');

            if !input.peek(Ident::peek_any) {
                return Err(input.error("expected identifier after '-' in attribute name"));
            }
            let ident: Ident = input.call(Ident::parse_any)?;
            name.push_str(&ident.to_string());
            last_span = Some(ident.span());
        } else if input.peek(Colon) {
            let _: Colon = input.parse()?;
            name.push(':');

            if !input.peek(Ident::peek_any) {
                return Err(input.error("expected identifier after ':' in attribute name"));
            }
            let ident: Ident = input.call(Ident::parse_any)?;
            name.push_str(&ident.to_string());
            last_span = Some(ident.span());
        } else if input.peek(Dot) {
            if input.peek2(Dot) {
                // Start of a spread pattern
                break;
            }

            let _: Dot = input.parse()?;
            name.push('.');

            if !input.peek(Ident::peek_any) {
                return Err(input.error("expected identifier after '.' in attribute name"));
            }
            let ident: Ident = input.call(Ident::parse_any)?;
            name.push_str(&ident.to_string());
            last_span = Some(ident.span());
        } else {
            break;
        }
    }

    let first = first_span.unwrap();
    let last = last_span.unwrap();
    let span = first.join(last).unwrap_or(first);

    Ok(AttributeName { name, span })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenTree;

    fn parse(input: &str) -> syn::Result<AttributeName> {
        syn::parse_str(input)
    }

    /// Helper to parse and return both the result and remaining tokens
    fn parse_partial(input: &str) -> syn::Result<(AttributeName, String)> {
        use std::str::FromStr;

        syn::parse::Parser::parse2(
            |stream: syn::parse::ParseStream| {
                let attr = parse_attribute_name(stream)?;
                let remaining = stream.cursor().token_stream().to_string();
                // Consume remaining to avoid "unexpected token" error
                while !stream.is_empty() {
                    let _: TokenTree = stream.parse()?;
                }
                Ok((attr, remaining))
            },
            proc_macro2::TokenStream::from_str(input).unwrap(),
        )
    }

    // === Valid attribute names ===

    #[test]
    fn simple_identifier() {
        assert_eq!(parse("class").unwrap().name, "class");
        assert_eq!(parse("href").unwrap().name, "href");
        assert_eq!(parse("id").unwrap().name, "id");
        assert_eq!(parse("_private").unwrap().name, "_private");
    }

    #[test]
    fn with_hyphens() {
        assert_eq!(parse("data-value").unwrap().name, "data-value");
        assert_eq!(parse("hx-get").unwrap().name, "hx-get");
        assert_eq!(parse("x-data").unwrap().name, "x-data");
        assert_eq!(parse("aria-label").unwrap().name, "aria-label");
    }

    #[test]
    fn with_colons() {
        assert_eq!(parse("hx-on:click").unwrap().name, "hx-on:click");
        assert_eq!(parse("xml:lang").unwrap().name, "xml:lang");
        assert_eq!(parse("xlink:href").unwrap().name, "xlink:href");
    }

    #[test]
    fn with_dots() {
        assert_eq!(parse("x.data").unwrap().name, "x.data");
        assert_eq!(parse("v.model").unwrap().name, "v.model");
    }

    #[test]
    fn at_prefixed() {
        assert_eq!(parse("@click").unwrap().name, "@click");
        assert_eq!(parse("@submit").unwrap().name, "@submit");
        assert_eq!(parse("@input").unwrap().name, "@input");
    }

    #[test]
    fn dollar_prefixed() {
        assert_eq!(parse("$var").unwrap().name, "$var");
        assert_eq!(parse("$data").unwrap().name, "$data");
    }

    #[test]
    fn colon_prefixed() {
        // Vue shorthand for v-bind
        assert_eq!(parse(":class").unwrap().name, ":class");
        assert_eq!(parse(":style").unwrap().name, ":style");
    }

    #[test]
    fn pound_prefixed() {
        assert_eq!(parse("#ref").unwrap().name, "#ref");
        assert_eq!(parse("#anchor").unwrap().name, "#anchor");
    }

    #[test]
    fn complex_combinations() {
        assert_eq!(parse("@click.shift").unwrap().name, "@click.shift");
        assert_eq!(parse("@keyup.page-down").unwrap().name, "@keyup.page-down");
        assert_eq!(
            parse("x-on:click.prevent").unwrap().name,
            "x-on:click.prevent"
        );
        assert_eq!(
            parse("hx-on:htmx:after-request").unwrap().name,
            "hx-on:htmx:after-request"
        );
        assert_eq!(
            parse("@click.stop.prevent").unwrap().name,
            "@click.stop.prevent"
        );
    }

    // === Error cases: empty input ===

    #[test]
    fn error_empty_input() {
        let result = parse("");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected attribute name")
        );
    }

    // === Error cases: invalid starting characters ===

    #[test]
    fn error_starts_with_hyphen() {
        let result = parse("-data");
        assert!(result.is_err());
    }

    #[test]
    fn error_starts_with_dot() {
        let result = parse(".class");
        assert!(result.is_err());
    }

    #[test]
    fn error_starts_with_equals() {
        let result = parse("=");
        assert!(result.is_err());
    }

    #[test]
    fn error_starts_with_greater_than() {
        let result = parse(">");
        assert!(result.is_err());
    }

    #[test]
    fn error_starts_with_slash() {
        let result = parse("/");
        assert!(result.is_err());
    }

    #[test]
    fn error_starts_with_less_than() {
        let result = parse("<");
        assert!(result.is_err());
    }

    // === Error cases: prefix without identifier ===

    #[test]
    fn error_at_only() {
        let result = parse("@");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected identifier after '@'")
        );
    }

    #[test]
    fn error_dollar_only() {
        let result = parse("$");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected identifier after '$'")
        );
    }

    #[test]
    fn error_colon_only() {
        let result = parse(":");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected identifier after ':'")
        );
    }

    #[test]
    fn error_pound_only() {
        let result = parse("#");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected identifier after '#'")
        );
    }

    // === Error cases: trailing punctuation ===

    #[test]
    fn error_trailing_hyphen() {
        let result = parse("data-");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected identifier after '-'")
        );
    }

    #[test]
    fn error_trailing_colon() {
        let result = parse("hx-on:");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected identifier after ':'")
        );
    }

    #[test]
    fn error_trailing_dot() {
        let result = parse("click.");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected identifier after '.'")
        );
    }

    // === Stops at disallowed characters ===

    #[test]
    fn stops_at_equals() {
        let (attr, remaining) = parse_partial("data=value").unwrap();
        assert_eq!(attr.name, "data");
        assert!(remaining.starts_with("="), "remaining was: {}", remaining);
    }

    #[test]
    fn stops_at_greater_than() {
        let (attr, remaining) = parse_partial("class>").unwrap();
        assert_eq!(attr.name, "class");
        assert!(remaining.starts_with(">"), "remaining was: {}", remaining);
    }

    #[test]
    fn stops_at_slash() {
        let (attr, remaining) = parse_partial("disabled/").unwrap();
        assert_eq!(attr.name, "disabled");
        assert!(remaining.starts_with("/"), "remaining was: {}", remaining);
    }

    #[test]
    fn stops_at_disallowed_punct() {
        // Stops at punctuation that's not -, :, or .
        let (attr, remaining) = parse_partial("data&more").unwrap();
        assert_eq!(attr.name, "data");
        assert!(remaining.starts_with("&"), "remaining was: {}", remaining);
    }

    // === Whitespace handling ===

    #[test]
    fn spaced_hyphen() {
        // "hx - get" has spaces in source, but Rust's tokenizer strips whitespace.
        // Token stream contains: [Ident(hx), Punct(-), Ident(get)]
        // Parser consumes all three and concatenates them.
        let (attr, remaining) = parse_partial("hx - get").unwrap();
        assert_eq!(attr.name, "hx-get");
        assert!(remaining.is_empty());
    }
}
