use std::fmt;

/// Trait for reusable HTML components.
///
/// A component is a renderable unit that accepts extra HTML attributes and children from its call site. You normally
/// don't implement this trait by hand - use the [`component!`](crate::component) macro instead, which generates a
/// struct and this trait implementation for you.
///
/// # The `render_component` method
///
/// The `attrs` closure writes any extra HTML attributes passed at the call site (those appearing after the `;` in
/// `@Component(props; attrs)`). The `children` closure writes the child content placed inside the component's braces.
///
/// # Example
///
/// ```
/// use plait::{component, html, ToHtml, classes, Class};
///
/// component! {
///     pub fn Alert(class: impl Class) {
///         div(class: classes!("alert", class), #attrs) {
///             #children
///         }
///     }
/// }
///
/// let page = html! {
///     @Alert(class: "alert-danger"; role: "alert") {
///         "Something went wrong!"
///     }
/// };
///
/// assert_eq!(page.to_html(), r#"<div class="alert alert-danger" role="alert">Something went wrong!</div>"#);
/// ```
pub trait Component {
    /// Renders the component, writing HTML into `f`.
    ///
    /// * `attrs` — closure that writes extra HTML attributes from the call site.
    /// * `children` — closure that writes child content from the call site.
    fn render_component(
        &self,
        f: &mut (dyn fmt::Write + '_),
        attrs: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
        children: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
    ) -> fmt::Result;
}

impl<T> Component for &T
where
    T: Component,
{
    fn render_component(
        &self,
        f: &mut (dyn fmt::Write + '_),
        attrs: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
        children: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
    ) -> fmt::Result {
        (**self).render_component(f, attrs, children)
    }
}
