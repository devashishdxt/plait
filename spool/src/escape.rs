mod html;

pub use self::html::escape_html;

/// Specifies how to escape the input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeMode {
    /// Don't escape the input.
    Raw,

    /// Escape the input as HTML (for html content and attributes).
    Html,
}

/// Resolves the escape mode for an element based on element name.
pub fn resolve_escape_mode_for_element(
    _name: Option<&str>,
    provided: Option<EscapeMode>,
) -> EscapeMode {
    provided.unwrap_or(EscapeMode::Html) // TODO: filter based on name, for example, `script` should be `JS`
}

/// Resolves the escape mode for an attribute based on attribute name and parent element name.
pub fn resolve_escape_mode_for_attribute(
    _parent: &str,
    _name: &str,
    provided: Option<EscapeMode>,
) -> EscapeMode {
    provided.unwrap_or(EscapeMode::Html) // TODO: filter based on name
}
