use core::{fmt, ops::Deref};

/// The HTML5 doctype declaration: `<!DOCTYPE html>`.
///
/// Use this constant at the beginning of your HTML documents.
pub const DOCTYPE: PreEscaped<'static> = PreEscaped("<!DOCTYPE html>");

/// A borrowed string slice that is known to contain safe, pre-escaped HTML.
///
/// Unlike regular strings which are escaped when rendered, `PreEscaped` content
/// is included in the output verbatim. This is useful for including HTML that
/// has already been escaped or for trusted HTML content.
///
/// # Safety
///
/// The caller must ensure that the string content is safe HTML. Including
/// unescaped user input via `PreEscaped` can lead to XSS vulnerabilities.
///
/// # Example
///
/// ```rust
/// use plait::PreEscaped;
///
/// // Include pre-escaped HTML content
/// let bold = PreEscaped("<strong>Important</strong>");
/// ```
///
/// # See Also
///
/// - [`Html`] - An owned version for dynamically constructed HTML
///
/// [`Html`]: crate::Html
pub struct PreEscaped<'a>(pub &'a str);

impl Deref for PreEscaped<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl AsRef<str> for PreEscaped<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl AsRef<[u8]> for PreEscaped<'_> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl fmt::Display for PreEscaped<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
