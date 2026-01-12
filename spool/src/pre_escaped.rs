use core::{fmt, ops::Deref};

/// `<!DOCTYPE html>`
pub const DOCTYPE: PreEscaped<'static> = PreEscaped("<!DOCTYPE html>");

/// A pre-escaped string slice known to contain safe HTML.
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
