use std::{borrow::Cow, fmt, ops::Deref};

use crate::{RenderEscaped, RenderRaw};

/// An owned string of rendered HTML.
///
/// `Html` is a thin wrapper around [`String`] that represents already-rendered HTML content. It implements
/// [`Deref<Target = str>`](Deref), [`Display`](fmt::Display), and can be converted back into a [`String`] with
/// [`From`].
///
/// You typically obtain an `Html` value by calling [`ToHtml::to_html()`] on an [`HtmlFragment`](crate::HtmlFragment)
/// returned by the [`html!`](crate::html) macro.
///
/// Because the content is already rendered HTML, both [`RenderEscaped`] and [`RenderRaw`] write the inner string as-is
/// (no double-escaping).
///
/// # Example
///
/// ```
/// use plait::{html, ToHtml};
///
/// let fragment = html! { p { "Hello" } };
/// let html = fragment.to_html();
///
/// assert_eq!(html, "<p>Hello</p>");
/// assert_eq!(html.to_string(), "<p>Hello</p>");
/// ```
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

#[cfg(feature = "actix-web")]
mod actix_web {
    use ::actix_web::{HttpRequest, HttpResponse, Responder};

    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "actix-web")))]
    impl Responder for Html {
        type Body = String;

        fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
            ::actix_web::web::Html::new(self).respond_to(req)
        }
    }
}

#[cfg(feature = "axum")]
mod axum {
    use ::axum::response::{IntoResponse, Response};

    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
    impl IntoResponse for Html {
        fn into_response(self) -> Response {
            ::axum::response::Html(String::from(self)).into_response()
        }
    }
}

#[cfg(feature = "rocket")]
mod rocket {
    use ::rocket::{
        Request,
        response::{Responder, Result, content::RawHtml},
    };

    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "rocket")))]
    impl<'r, 'o: 'r> Responder<'r, 'o> for Html {
        fn respond_to(self, request: &'r Request<'_>) -> Result<'o> {
            RawHtml(String::from(self)).respond_to(request)
        }
    }
}

/// Trait for types that can be rendered into an [`Html`] value.
///
/// This is the primary way to materialize a template into an owned HTML string. The [`html!`](crate::html) macro
/// returns an [`HtmlFragment`](crate::HtmlFragment) that implements this trait.
///
/// # Example
///
/// ```
/// use plait::{html, ToHtml};
///
/// let fragment = html! { div { "Hello" } };
/// let html = fragment.to_html(); // Html value
/// println!("{html}");            // prints: <div>Hello</div>
/// ```
pub trait ToHtml {
    /// Renders `self` into an [`Html`] value.
    fn to_html(&self) -> Html;
}
