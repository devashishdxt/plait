use core::{fmt, ops::Deref};

use crate::PreEscaped;

/// A html string known to contain safe HTML.
pub struct Html(pub String);

impl Html {
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

impl From<Html> for String {
    fn from(value: Html) -> Self {
        value.0
    }
}
