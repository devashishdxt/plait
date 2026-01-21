mod html;
mod url;

pub use self::{html::escape_html, url::escape_url};

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

/// Specifies how content should be escaped when rendered to HTML.
///
/// The escape mode determines what transformations are applied to content
/// before it is included in the HTML output. Choosing the correct escape
/// mode is important for both security and correctness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeMode {
    /// Don't escape the input.
    Raw,

    /// Escape the input as HTML (for html content and attributes).
    Html,

    /// Escape the input as a URL (for URL attributes).
    Url,
}

/// Resolves the escape mode for an element based on element name.
pub fn resolve_escape_mode_for_element(
    _name: Option<&str>,
    provided: Option<EscapeMode>,
) -> EscapeMode {
    provided.unwrap_or(EscapeMode::Html) // TODO: filter based on name, for example, `script` should be `Js`
}

/// Resolves the escape mode for an attribute based on attribute name.
pub fn resolve_escape_mode_for_attribute(name: &str, provided: Option<EscapeMode>) -> EscapeMode {
    provided.unwrap_or_else(|| {
        if is_url_attribute(name) {
            EscapeMode::Url
        } else {
            EscapeMode::Html
        }
    })
}
