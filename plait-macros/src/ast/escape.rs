use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

/// Specifies how to escape content in the template macro AST.
///
/// This is the macro-crate version of the escape mode, used during parsing.
/// It maps to the runtime [`plait::EscapeMode`] during code generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeMode {
    /// Don't escape the input. Use for pre-escaped or trusted content.
    Raw,

    /// Escape the input as HTML. This is the default for dynamic content.
    Html,
}

impl Parse for EscapeMode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        parse_escape_mode(input)
    }
}

/// Parses an escape mode identifier from the input stream.
///
/// Accepts `raw` or `html` as valid escape modes.
///
/// # Errors
///
/// Returns an error if the identifier is not a recognized escape mode.
pub fn parse_escape_mode(input: ParseStream<'_>) -> syn::Result<EscapeMode> {
    let ident: Ident = input.parse()?;

    match ident.to_string().as_str() {
        "raw" => Ok(EscapeMode::Raw),
        "html" => Ok(EscapeMode::Html),
        _ => Err(syn::Error::new(ident.span(), "invalid escape mode")),
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn test_parse_escape_mode_raw() {
        let input = quote! { raw };
        let mode: EscapeMode = syn::parse2(input).unwrap();
        assert_eq!(mode, EscapeMode::Raw);
    }

    #[test]
    fn test_parse_escape_mode_html() {
        let input = quote! { html };
        let mode: EscapeMode = syn::parse2(input).unwrap();
        assert_eq!(mode, EscapeMode::Html);
    }

    #[test]
    fn test_parse_escape_mode_invalid() {
        let input = quote! { invalid };
        let result: syn::Result<EscapeMode> = syn::parse2(input);
        assert!(result.is_err());
    }
}
