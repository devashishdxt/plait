use core::fmt;

use crate::display::HtmlDisplay;

/// The concrete type returned by the [`html!`](crate::html) macro.
///
/// `HtmlFragment` wraps a closure that writes HTML into any [`fmt::Write`] sink. It implements both
/// [`Display`](core::fmt::Display) and [`HtmlDisplay`], so it can be rendered with `.to_string()` / `write!` and
/// embedded in other templates via `@(expr)`.
///
/// You do not need to construct `HtmlFragment` directly - it is created by [`html!`](crate::html).
///
/// # Example
///
/// ```rust
/// use plait::html;
///
/// let inner = html! { em { "world" } };
/// let outer = html! { p { "hello " @(&inner) } };
///
/// assert_eq!(outer.to_string(), "<p>hello <em>world</em></p>");
/// ```
pub struct HtmlFragment<F>(pub F)
where
    F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result;

impl<F> HtmlDisplay for HtmlFragment<F>
where
    F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
{
    fn html_fmt(&self, w: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        (self.0)(w)
    }
}

impl<F> fmt::Display for HtmlFragment<F>
where
    F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}
