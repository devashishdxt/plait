use crate::{
    EscapeMode,
    escape::{escape_html, escape_url},
};

use super::{Html, PreEscaped};

/// A trait for types that can be rendered as HTML content.
///
/// This trait is implemented for common types like strings, numbers, and booleans.
/// The rendering process handles escaping based on the provided [`EscapeMode`].
///
/// # Implementations
///
/// Built-in implementations include:
/// - `&str`, `String` - Escaped according to the escape mode
/// - Integer types (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`)
/// - Float types (`f32`, `f64`)
/// - `bool` - Renders as `"true"` or `"false"`
/// - `char` - Escaped according to the escape mode
/// - [`PreEscaped`] - Rendered without additional escaping
/// - [`Html`] - Rendered without additional escaping
/// - `Option<T>` - Renders the inner value if `Some`, nothing if `None`
/// - `&T`, `&mut T`, `Box<T>` where `T: Render`
///
/// # Implementing for Custom Types
///
/// You can implement `Render` for your own types to create reusable components.
/// Use [`HtmlFormatter`] and the [`render!`] macro to generate the HTML output:
///
/// ```rust
/// use plait::{EscapeMode, Html, HtmlFormatter, Render, render};
///
/// struct UserCard {
///     name: String,
///     email: String,
/// }
///
/// impl Render for UserCard {
///     fn render_to(&self, output: &mut Html, _escape_mode: EscapeMode) {
///         let mut fmt = HtmlFormatter::new(output);
///         render!(fmt, {
///             div class="user-card" {
///                 h2 { (&self.name) }
///                 p class="email" { (&self.email) }
///             }
///         });
///     }
/// }
///
/// // Now UserCard can be used in templates:
/// let user = UserCard {
///     name: "Alice".into(),
///     email: "alice@example.com".into(),
/// };
/// let html = plait::html!(
///     div class="users" {
///         (user)
///     }
/// );
/// ```
///
/// [`EscapeMode`]: crate::EscapeMode
/// [`PreEscaped`]: crate::PreEscaped
/// [`Html`]: crate::Html
/// [`HtmlFormatter`]: crate::HtmlFormatter
/// [`render!`]: crate::render
pub trait Render {
    /// Renders the HTML content into the provided output.
    fn render_to(&self, output: &mut Html, escape_mode: EscapeMode);

    /// Renders the HTML content into a new `Html` instance.
    fn render(&self, escape_mode: EscapeMode) -> Html {
        let mut output = Html::new();
        self.render_to(&mut output, escape_mode);
        output
    }
}

impl Render for str {
    fn render_to(&self, output: &mut Html, escape_mode: EscapeMode) {
        match escape_mode {
            EscapeMode::Raw => output.0.push_str(self),
            EscapeMode::Html => escape_html(output, self),
            EscapeMode::Url => escape_url(output, self),
        }
    }
}

impl Render for String {
    fn render_to(&self, output: &mut Html, escape_mode: EscapeMode) {
        self.as_str().render_to(output, escape_mode)
    }
}

impl Render for PreEscaped<'_> {
    fn render_to(&self, output: &mut Html, _escape_mode: EscapeMode) {
        output.0.push_str(self.0);
    }
}

impl Render for Html {
    fn render_to(&self, output: &mut Html, _escape_mode: EscapeMode) {
        output.0.push_str(&self.0);
    }
}

macro_rules! impl_render_for_int {
    ($($ty:ty),*) => {
        $(
            impl Render for $ty {
                fn render_to(
                    &self,
                    output: &mut Html,
                    _escape_mode: EscapeMode,
                ) {
                    let mut buf = itoa::Buffer::new();
                    output.0.push_str(buf.format(*self));
                }
            }
        )*
    };
}

impl_render_for_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

macro_rules! impl_render_for_float {
    ($($ty:ty),*) => {
        $(
            impl Render for $ty {
                fn render_to(
                    &self,
                    output: &mut Html,
                    _escape_mode: EscapeMode,
                ) {
                    let mut buf = ryu::Buffer::new();
                    output.0.push_str(buf.format(*self));
                }
            }
        )*
    };
}

impl_render_for_float!(f32, f64);

impl Render for bool {
    fn render_to(&self, output: &mut Html, _escape_mode: EscapeMode) {
        output.0.push_str(if *self { "true" } else { "false" });
    }
}

impl Render for char {
    fn render_to(&self, output: &mut Html, escape_mode: EscapeMode) {
        let mut buf = [0u8; 4];
        let s = self.encode_utf8(&mut buf);
        s.render_to(output, escape_mode)
    }
}

impl<T> Render for &T
where
    T: Render + ?Sized,
{
    fn render_to(&self, output: &mut Html, escape_mode: EscapeMode) {
        (**self).render_to(output, escape_mode)
    }
}

impl<T> Render for &mut T
where
    T: Render + ?Sized,
{
    fn render_to(&self, output: &mut Html, escape_mode: EscapeMode) {
        (**self).render_to(output, escape_mode)
    }
}

impl<T> Render for Box<T>
where
    T: Render + ?Sized,
{
    fn render_to(&self, output: &mut Html, escape_mode: EscapeMode) {
        (**self).render_to(output, escape_mode)
    }
}

impl<T> Render for Option<T>
where
    T: Render,
{
    fn render_to(&self, output: &mut Html, escape_mode: EscapeMode) {
        if let Some(this) = self {
            this.render_to(output, escape_mode)
        }
    }
}
