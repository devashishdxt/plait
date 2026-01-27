use core::{fmt, ops::Deref};

/// The HTML5 doctype declaration: `<!DOCTYPE html>`.
pub const DOCTYPE: PreEscaped<'static> = PreEscaped("<!DOCTYPE html>");

/// A borrowed string slice that is known to contain safe, pre-escaped HTML.
///
/// Unlike regular strings which are escaped when rendered, `PreEscaped` content is included in the output verbatim.
/// This is useful for including HTML that has already been escaped or for trusted HTML content.
///
/// # Safety
///
/// The caller must ensure that the string content is safe HTML. Including unescaped user input via `PreEscaped` can
/// lead to XSS vulnerabilities.
///
/// # Example
///
/// ```rust
/// use plait::{PreEscaped, render};
///
/// // Include pre-escaped HTML content
/// let bold = PreEscaped("<strong>Important</strong>");
///
/// assert_eq!(render(bold), "<strong>Important</strong>");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreEscaped<'a>(pub &'a str);

impl Deref for PreEscaped<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl fmt::Display for PreEscaped<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
