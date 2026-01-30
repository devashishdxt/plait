use core::{fmt, ops::Deref};

/// A wrapper type representing safe, pre-rendered HTML content.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Html(String);

impl Html {
    /// Create a new empty HTML string.
    pub fn new() -> Self {
        Html(String::new())
    }

    /// Create a new HTML string with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Html(String::with_capacity(capacity))
    }

    /// Convert the HTML string into a `String`.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Get a mutable reference to the inner `String`.
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
    fn from(html: Html) -> Self {
        html.0
    }
}

impl PartialEq<&str> for Html {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
