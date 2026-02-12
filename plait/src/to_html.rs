use core::fmt::{Display, Write};

use crate::{Html, HtmlFormatter, HtmlFragment};

/// A trait for types that can render themselves as HTML content with automatic escaping.
///
/// This trait is used by the [`html!`](crate::html) macro for expressions in `(...)` syntax. Content is HTML-escaped
/// to prevent XSS vulnerabilities.
///
/// # Implementations
///
/// - **[`HtmlFragment`]** - Renders the fragment directly (the fragment itself handles escaping)
/// - **[`Html`]** - Renders the HTML string directly (`Html` is already escaped)
/// - **[`Display`] types** - Rendered with HTML escaping (`<`, `>`, `&`, `'`, `"` are escaped)
///
/// # Examples
///
/// ```rust
/// use plait::{html, render};
///
/// let user_input = "<script>alert('xss')</script>";
/// let html = render(html! { p { (user_input) } });
///
/// // Content is escaped
/// assert_eq!(html, "<p>&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;</p>");
/// ```
///
/// Components can accept any `IntoHtml` type as a prop:
///
/// ```rust
/// use plait::{IntoHtml, component, html, render};
///
/// component! {
///     fn Wrapper(content: impl IntoHtml) {
///         div(class: "wrapper") { (content) }
///     }
/// }
///
/// // Pass a string
/// let html1 = render(html! { @Wrapper(content: "Hello") {} });
/// assert_eq!(html1, "<div class=\"wrapper\">Hello</div>");
///
/// // Pass an html! fragment
/// let html2 = render(html! { @Wrapper(content: html! { strong { "Bold" } }) {} });
/// assert_eq!(html2, "<div class=\"wrapper\"><strong>Bold</strong></div>");
/// ```
pub trait IntoHtml {
    /// Renders this value to the given HTML formatter with escaping.
    fn render_to(self, f: &mut HtmlFormatter<'_>);
}

impl<F> IntoHtml for HtmlFragment<F>
where
    F: FnOnce(&mut HtmlFormatter<'_>),
{
    fn render_to(self, f: &mut HtmlFormatter<'_>) {
        (self.0)(f);
    }
}

impl IntoHtml for Html {
    fn render_to(self, f: &mut HtmlFormatter<'_>) {
        write!(f.raw_writer(), "{}", self.into_string()).unwrap()
    }
}

impl IntoHtml for &Html {
    fn render_to(self, f: &mut HtmlFormatter<'_>) {
        write!(f.raw_writer(), "{}", self.as_str()).unwrap()
    }
}

impl<T> IntoHtml for T
where
    T: Display,
{
    fn render_to(self, f: &mut HtmlFormatter<'_>) {
        write!(f.html_escaped_writer(), "{self}").unwrap()
    }
}

/// A trait for types that can render themselves as raw HTML content without escaping.
///
/// This trait is used by the [`html!`](crate::html) macro for expressions in `#(...)` syntax. Content is written
/// directly without HTML escaping.
///
/// # Safety
///
/// **Use with caution.** Only use this for trusted content. Rendering untrusted user input through this trait can
/// lead to XSS vulnerabilities.
///
/// # Implementations
///
/// - **[`HtmlFragment`]** - Renders the fragment directly (same behavior as [`IntoHtml`])
/// - **[`Html`]** - Renders the HTML string directly (same behavior as [`IntoHtml`])
/// - **[`Display`] types** - Rendered without escaping
///
/// # Examples
///
/// ```rust
/// use plait::{html, render};
///
/// let trusted_html = "<strong>Bold</strong>";
/// let html = render(html! { div { #(trusted_html) } });
///
/// // Content is NOT escaped
/// assert_eq!(html, "<div><strong>Bold</strong></div>");
/// ```
pub trait IntoHtmlRaw {
    /// Renders this value to the given HTML formatter without escaping.
    fn render_raw_to(self, f: &mut HtmlFormatter<'_>);
}

impl<F> IntoHtmlRaw for HtmlFragment<F>
where
    F: FnOnce(&mut HtmlFormatter<'_>),
{
    fn render_raw_to(self, f: &mut HtmlFormatter<'_>) {
        (self.0)(f);
    }
}

impl IntoHtmlRaw for Html {
    fn render_raw_to(self, f: &mut HtmlFormatter<'_>) {
        write!(f.raw_writer(), "{}", self.into_string()).unwrap()
    }
}

impl IntoHtmlRaw for &Html {
    fn render_raw_to(self, f: &mut HtmlFormatter<'_>) {
        write!(f.raw_writer(), "{}", self.as_str()).unwrap()
    }
}

impl<T> IntoHtmlRaw for T
where
    T: Display,
{
    fn render_raw_to(self, f: &mut HtmlFormatter<'_>) {
        write!(f.raw_writer(), "{self}").unwrap()
    }
}
