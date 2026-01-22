use crate::{EscapeMode, HtmlFormatter, Render};

/// A lazy component that defers rendering until embedded in a parent template.
///
/// `LazyRender` wraps a closure that performs rendering when called. This allows you to create reusable components as
/// functions that return `impl Render`, where the actual rendering happens lazily using the parent's
/// [`HtmlFormatter`] instead of creating an intermediate buffer.
///
/// # Creating Components
///
/// Use the [`component!`] macro to create `LazyRender` instances:
///
/// ```rust
/// use plait::{Render, component};
///
/// pub fn button(label: &str, primary: bool) -> impl Render + '_ {
///     let class = if primary { "btn btn-primary" } else { "btn" };
///     component! {
///         button class=(class) { (label) }
///     }
/// }
///
/// // Use in templates
/// let btn = button("Click me", true);
/// let output = plait::html!(
///     div { (btn) }
/// );
///
/// assert_eq!(&*output, r#"<div><button class="btn btn-primary">Click me</button></div>"#);
/// ```
///
/// # Why Use `LazyRender`?
///
/// When you use [`html!`] inside a component function, it eagerly renders to a new `Html` buffer. When that `Html` is
/// embedded in a parent template, it simply copies the pre-rendered string.
///
/// With `LazyRender` (via [`component!`]), the rendering is deferred. When the component is embedded in a parent
/// template, it renders directly to the parent's formatter, avoiding intermediate allocations.
///
/// # Borrowing in Components
///
/// Components may be rendered multiple times, so the internal closure must implement `Fn` (not just `FnOnce`). When
/// using owned values in expressions, use `(&value)` to borrow instead of move:
///
/// ```rust
/// use plait::{Render, component};
///
/// pub fn greeting(label: String) -> impl Render {
///     component! {
///         span { (&label) }  // borrow with (&...) to allow multiple renders
///     }
/// }
/// ```
///
/// For components that take references, include the lifetime in the return type:
///
/// ```rust
/// use plait::{Render, component};
///
/// pub fn greeting(label: &str) -> impl Render + '_ {
///     component! {
///         span { (label) }
///     }
/// }
/// ```
///
/// [`component!`]: crate::component
/// [`html!`]: crate::html
/// [`HtmlFormatter`]: crate::HtmlFormatter
pub struct LazyRender<F>(pub F);

impl<F> Render for LazyRender<F>
where
    F: Fn(&mut HtmlFormatter),
{
    fn render_to(&self, f: &mut HtmlFormatter, _escape_mode: EscapeMode) {
        (self.0)(f)
    }
}
