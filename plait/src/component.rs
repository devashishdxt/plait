use core::fmt;

/// A reusable HTML component with typed props, attribute spreading, and child projection.
///
/// This trait is automatically implemented by the [`component!`](crate::component) macro - you should not need to
/// implement it manually. The generated implementation calls `html_fmt` with closures that render any extra HTML
/// attributes (`#attrs`) and child content (`#children`) supplied at the call site.
///
/// # Calling components
///
/// Components are invoked inside [`html!`](crate::html) with the `@Name(...)` syntax. Props and extra HTML attributes
/// are separated by a semicolon - props come before the `;`, attributes after:
///
/// ```text
/// @Button(class: "primary"; id: "btn", disabled?: true) { "Click" }
/// //      ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// //            props             HTML attributes
/// ```
///
/// Extra HTML attributes are always optional. If none are needed, the semicolon can be omitted:
///
/// ```text
/// @Button(class: "primary") { "Click" }
/// ```
///
/// # Example
///
/// ```rust
/// use plait::{component, html, classes, ClassPart};
///
/// component! {
///     fn Card(class: impl ClassPart) {
///         div(class: classes!("card", class), #attrs) {
///             #children
///         }
///     }
/// }
///
/// let html = html! {
///     @Card(class: "highlighted"; id: "card-1") {
///         p { "Card body" }
///     }
/// };
///
/// assert_eq!(
///     html.to_string(),
///     "<div class=\"card highlighted\" id=\"card-1\"><p>Card body</p></div>",
/// );
/// ```
pub trait Component {
    fn html_fmt(
        &self,
        w: &mut (dyn fmt::Write + '_),
        attrs: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
        children: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
    ) -> fmt::Result;
}

impl<T> Component for &T
where
    T: Component,
{
    fn html_fmt(
        &self,
        w: &mut (dyn fmt::Write + '_),
        attrs: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
        children: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
    ) -> fmt::Result {
        (**self).html_fmt(w, attrs, children)
    }
}
