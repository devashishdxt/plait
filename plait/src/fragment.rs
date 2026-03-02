use std::fmt;

use crate::{Html, RenderEscaped, ToHtml};

/// A lazy HTML fragment returned by the [`html!`](crate::html) macro.
///
/// An `HtmlFragment` wraps a closure that writes HTML into a [`fmt::Write`] buffer. It carries a `size_hint` used to
/// pre-allocate the output string for better performance.
///
/// Call [`to_html()`](ToHtml::to_html) to materialize the fragment into an [`Html`] value, or embed it inside another
/// `html!` template using `(fragment)`.
///
/// # Example
///
/// ```
/// use plait::{html, ToHtml};
///
/// let header = html! { h1 { "Title" } };
/// let page = html! {
///     div {
///         (header)
///         p { "Body" }
///     }
/// };
///
/// assert_eq!(page.to_html(), "<div><h1>Title</h1><p>Body</p></div>");
/// ```
pub struct HtmlFragment<F>
where
    F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
{
    f: F,
    size_hint: usize,
}

impl<F> HtmlFragment<F>
where
    F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
{
    #[doc(hidden)]
    pub fn new(f: F, size_hint: usize) -> Self {
        HtmlFragment { f, size_hint }
    }

    fn to_string(&self) -> String {
        let mut buffer = String::with_capacity(self.size_hint);
        (self.f)(&mut buffer).unwrap();
        buffer
    }
}

impl<F> RenderEscaped for HtmlFragment<F>
where
    F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
{
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        (self.f)(f)
    }
}

impl<F> ToHtml for HtmlFragment<F>
where
    F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
{
    fn to_html(&self) -> Html {
        Html::new_unchecked(self.to_string())
    }
}

/// Marker trait for types that represent partial HTML content.
///
/// `PartialHtml` is a subtrait of [`RenderEscaped`] intended for use as a component prop bound when the prop should
/// accept an [`HtmlFragment`] (i.e. the output of [`html!`](crate::html)). This is more descriptive than using
/// `RenderEscaped` directly, and signals that the prop expects rendered HTML rather than plain text.
///
/// # Example
///
/// ```
/// use plait::{component, html, ToHtml, PartialHtml};
///
/// component! {
///     pub fn Card(title: impl PartialHtml) {
///         div(class: "card") {
///             h1 { (title) }
///             #children
///         }
///     }
/// }
///
/// let page = html! {
///     @Card(title: html! { span { "My Card" } }) {
///         p { "Card body" }
///     }
/// };
///
/// assert_eq!(
///     page.to_html(),
///     r#"<div class="card"><h1><span>My Card</span></h1><p>Card body</p></div>"#
/// );
/// ```
pub trait PartialHtml: RenderEscaped {}

impl<F> PartialHtml for HtmlFragment<F> where F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result {}
