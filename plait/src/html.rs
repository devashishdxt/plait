use core::{fmt, ops::Deref};

use crate::PreEscaped;

/// An owned string of escaped HTML content.
///
/// `Html` is the primary output type for rendered HTML content. It guarantees that the content it contains has been
/// properly escaped (or was explicitly marked as pre-escaped). When rendered again, `Html` content is included
/// verbatim without additional escaping.
///
/// # Example
///
/// ```rust
/// use plait::{Html, render};
///
/// let html: Html = render("<script>alert('xss')</script>");
/// assert_eq!(html, "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;");
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Html(String);

impl Html {
    /// Creates a new empty `Html` string.
    pub fn new() -> Self {
        Html(String::new())
    }

    /// Creates a new `Html` string with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Html(String::with_capacity(capacity))
    }

    /// Returns a reference to the pre-escaped string.
    pub fn as_pre_escaped(&self) -> PreEscaped<'_> {
        PreEscaped(&self.0)
    }

    /// Returns a mutable reference to the inner string.
    pub(crate) fn inner_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl Deref for Html {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Html {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Html> for String {
    fn from(value: Html) -> Self {
        value.0
    }
}

impl<'a> From<&'a Html> for PreEscaped<'a> {
    fn from(value: &'a Html) -> Self {
        PreEscaped(&value.0)
    }
}

impl PartialEq<&str> for Html {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
