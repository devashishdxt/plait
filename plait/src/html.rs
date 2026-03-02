use std::{borrow::Cow, fmt, ops::Deref};

use crate::{RenderEscaped, RenderRaw};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Html(String);

impl Html {
    #[doc(hidden)]
    pub fn new_unchecked(s: String) -> Self {
        Html(s)
    }
}

impl Deref for Html {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Html> for String {
    fn from(html: Html) -> Self {
        html.0
    }
}

impl fmt::Display for Html {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<&str> for Html {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Cow<'_, str>> for Html {
    fn eq(&self, other: &Cow<'_, str>) -> bool {
        self.0 == *other
    }
}

impl RenderEscaped for Html {
    #[inline]
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl RenderRaw for Html {
    #[inline]
    fn render_raw(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        f.write_str(&self.0)
    }
}

pub trait ToHtml {
    fn to_html(&self) -> Html;
}
