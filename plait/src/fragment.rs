use crate::HtmlFormatter;

/// A wrapper type for HTML rendering closures returned by the [`html!`](crate::html) macro.
///
/// `HtmlFragment` wraps a closure that writes HTML to an [`HtmlFormatter`]. This type implements both
/// [`ToHtml`](crate::ToHtml) and [`ToHtmlRaw`](crate::ToHtmlRaw), allowing fragments to be:
///
/// - Passed to [`render`](crate::render) and [`render_with_capacity`](crate::render_with_capacity)
/// - Used as component props with generic `T: ToHtml` bounds
/// - Stored in variables and composed together
/// - Embedded in other [`html!`](crate::html) fragments via `(expr)` or `#(expr)` syntax
///
/// You typically don't create `HtmlFragment` directly - it's returned by the [`html!`](crate::html) macro.
///
/// # Examples
///
/// ```rust
/// use plait::{ToHtml, html, render};
///
/// // Store a fragment in a variable
/// let header = html! { h1 { "Welcome" } };
///
/// // Use it in another fragment
/// let page = render(html! {
///     div {
///         (header)
///         p { "Content" }
///     }
/// });
///
/// assert_eq!(page, "<div><h1>Welcome</h1><p>Content</p></div>");
/// ```
pub struct HtmlFragment<F>(pub F)
where
    F: FnOnce(&mut HtmlFormatter<'_>);
