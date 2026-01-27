use crate::{HtmlFormatter, Render};

/// A wrapper that allows closures to implement [`Render`].
///
/// This is the type returned by the [`html!`](crate::html) macro.
///
/// `RenderFn` enables using closures or function pointers as renderable content, providing a convenient way to create
/// inline rendering logic without defining a separate type.
///
/// The closure receives an [`HtmlFormatter`] and can use it to build arbitrary HTML content. Note that the same
/// closure is used for all escape modes (`render_html`, `render_url`, `render_raw`), so the closure is responsible
/// for any escaping if needed.
///
/// # Examples
///
/// Using the `html!` macro (recommended):
///
/// ```rust
/// use plait::{html, render};
///
/// let greeting = html! { p { "Hello, world!" } };
/// let html = render(greeting);
/// assert_eq!(html, "<p>Hello, world!</p>");
/// ```
///
/// Using `RenderFn` directly:
///
/// ```rust
/// use plait::{Html, HtmlFormatter, RenderFn, render};
///
/// let items = vec!["apple", "banana", "cherry"];
///
/// let list = RenderFn::from(|f: &mut HtmlFormatter| {
///     f.start_element("ul");
///     for item in &items {
///         f.start_element("li");
///         f.write_content(*item, None).unwrap();
///         f.end_element().unwrap();
///     }
///     f.end_element().unwrap();
/// });
///
/// let html = render(list);
/// assert_eq!(html, "<ul><li>apple</li><li>banana</li><li>cherry</li></ul>");
/// ```
pub struct RenderFn<F>(F)
where
    F: Fn(&mut HtmlFormatter);

impl<F> RenderFn<F>
where
    F: Fn(&mut HtmlFormatter),
{
    /// Creates a new `RenderFn` from a closure.
    pub fn new(f: F) -> Self {
        RenderFn(f)
    }
}

impl<F> From<F> for RenderFn<F>
where
    F: Fn(&mut HtmlFormatter),
{
    fn from(f: F) -> Self {
        RenderFn(f)
    }
}

impl<F> Render for RenderFn<F>
where
    F: Fn(&mut HtmlFormatter),
{
    fn render_html(&self, f: &mut HtmlFormatter) {
        (self.0)(f)
    }

    fn render_url(&self, f: &mut HtmlFormatter) {
        self.render_html(f);
    }

    fn render_raw(&self, f: &mut HtmlFormatter) {
        self.render_html(f);
    }
}
