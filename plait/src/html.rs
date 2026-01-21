use core::{fmt, ops::Deref};

use crate::PreEscaped;

/// An owned string that is known to contain safe, properly escaped HTML.
///
/// This type is the primary output type for HTML rendering operations. The inner string is guaranteed to be safe for
/// direct inclusion in HTML documents without additional escaping.
///
/// # Safety
///
/// The `Html` type assumes its contents are already properly escaped. Creating an `Html` instance with unescaped user
/// input could lead to XSS vulnerabilities. Use the [`Render`] trait or [`HtmlFormatter`] to safely construct HTML
/// content.
///
/// [`Render`]: crate::Render
/// [`HtmlFormatter`]: crate::HtmlFormatter
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Html(pub String);

impl Html {
    /// Creates a new empty `Html` string.
    pub fn new() -> Self {
        Html(String::new())
    }

    /// Returns a reference to the pre-escaped string.
    pub fn as_pre_escaped(&self) -> PreEscaped<'_> {
        PreEscaped(&self.0)
    }
}

impl Deref for Html {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Html {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<[u8]> for Html {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl fmt::Display for Html {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Write for Html {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}

impl From<Html> for String {
    fn from(value: Html) -> Self {
        value.0
    }
}

impl PartialEq<&str> for Html {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
