use crate::escape::{escape_html, escape_url};

use super::{EscapeMode, Html, HtmlFormatter, PreEscaped};

/// A trait for types that can be rendered to HTML.
///
/// Types implementing `Render` can be written to an [`HtmlFormatter`] with different escaping strategies depending on
/// the context. The trait provides three rendering methods corresponding to different escape modes:
///
/// - [`render_html`](Render::render_html): Escapes HTML special characters (`<`, `>`, `&`, `"`, `'`)
/// - [`render_url`](Render::render_url): Validates the URL scheme (blocking dangerous schemes like `javascript:`) and
///   applies HTML escaping
/// - [`render_raw`](Render::render_raw): Outputs content without any escaping
///
/// # Implementors
///
/// The library provides implementations for common types:
/// - `str`, `String`, `char` - escaped according to the render method
/// - Numeric types (`i32`, `u64`, `f64`, etc.) - rendered without escaping (safe)
/// - `bool` - rendered as `"true"` or `"false"`
/// - [`PreEscaped`] - always rendered without escaping
/// - [`Html`] - always rendered without escaping (already escaped)
/// - `Option<T>` - renders inner value if `Some`, nothing if `None`
/// - `&T`, `&mut T`, `Box<T>` - delegates to inner type
///
/// # Example
///
/// ```rust
/// use plait::{Html, HtmlFormatter, Render};
///
/// struct Username(String);
///
/// impl Render for Username {
///     fn render_html(&self, f: &mut HtmlFormatter) {
///         // Delegate to String's implementation which escapes HTML
///         self.0.render_html(f);
///     }
///
///     fn render_url(&self, f: &mut HtmlFormatter) {
///         self.0.render_url(f);
///     }
///
///     fn render_raw(&self, f: &mut HtmlFormatter) {
///         self.0.render_raw(f);
///     }
/// }
/// ```
pub trait Render {
    fn render_html(&self, f: &mut HtmlFormatter);

    fn render_url(&self, f: &mut HtmlFormatter);

    fn render_raw(&self, f: &mut HtmlFormatter);

    fn render_to(&self, f: &mut HtmlFormatter, escape_mode: EscapeMode) {
        match escape_mode {
            EscapeMode::Html => self.render_html(f),
            EscapeMode::Url => self.render_url(f),
            EscapeMode::Raw => self.render_raw(f),
        }
    }
}

impl Render for str {
    fn render_html(&self, f: &mut HtmlFormatter) {
        escape_html(f.output(), self);
    }

    fn render_url(&self, f: &mut HtmlFormatter) {
        escape_url(f.output(), self);
    }

    fn render_raw(&self, f: &mut HtmlFormatter) {
        f.output().inner_mut().push_str(self);
    }
}

impl Render for String {
    fn render_html(&self, f: &mut HtmlFormatter) {
        escape_html(f.output(), self);
    }

    fn render_url(&self, f: &mut HtmlFormatter) {
        escape_url(f.output(), self);
    }

    fn render_raw(&self, f: &mut HtmlFormatter) {
        f.output().inner_mut().push_str(self);
    }
}

impl Render for PreEscaped<'_> {
    fn render_html(&self, f: &mut HtmlFormatter) {
        self.render_raw(f);
    }

    fn render_url(&self, f: &mut HtmlFormatter) {
        self.render_raw(f);
    }

    fn render_raw(&self, f: &mut HtmlFormatter) {
        f.output().inner_mut().push_str(self);
    }
}

impl Render for Html {
    fn render_html(&self, f: &mut HtmlFormatter) {
        self.render_raw(f);
    }

    fn render_url(&self, f: &mut HtmlFormatter) {
        self.render_raw(f);
    }

    fn render_raw(&self, f: &mut HtmlFormatter) {
        f.output().inner_mut().push_str(self);
    }
}

macro_rules! impl_render_for_int {
    ($($ty:ty),*) => {
        $(
            impl Render for $ty {
                fn render_html(&self, f: &mut HtmlFormatter) {
                    self.render_raw(f);
                }

                fn render_url(&self, f: &mut HtmlFormatter) {
                    self.render_raw(f);
                }

                fn render_raw(&self, f: &mut HtmlFormatter) {
                    let mut buf = itoa::Buffer::new();
                    f.output().inner_mut().push_str(buf.format(*self));
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
                fn render_html(&self, f: &mut HtmlFormatter) {
                    self.render_raw(f);
                }

                fn render_url(&self, f: &mut HtmlFormatter) {
                    self.render_raw(f);
                }

                fn render_raw(&self, f: &mut HtmlFormatter) {
                    let mut buf = ryu::Buffer::new();
                    f.output().inner_mut().push_str(buf.format(*self));
                }
            }
        )*
    };
}

impl_render_for_float!(f32, f64);

impl Render for bool {
    fn render_html(&self, f: &mut HtmlFormatter) {
        self.render_raw(f);
    }

    fn render_url(&self, f: &mut HtmlFormatter) {
        self.render_raw(f);
    }

    fn render_raw(&self, f: &mut HtmlFormatter) {
        f.output()
            .inner_mut()
            .push_str(if *self { "true" } else { "false" });
    }
}

impl Render for char {
    fn render_html(&self, f: &mut HtmlFormatter) {
        let mut buf = [0u8; 4];
        let s = self.encode_utf8(&mut buf);
        s.render_html(f);
    }

    fn render_url(&self, f: &mut HtmlFormatter) {
        let mut buf = [0u8; 4];
        let s = self.encode_utf8(&mut buf);
        s.render_url(f);
    }

    fn render_raw(&self, f: &mut HtmlFormatter) {
        let mut buf = [0u8; 4];
        let s = self.encode_utf8(&mut buf);
        s.render_raw(f);
    }
}

impl<T> Render for &T
where
    T: Render + ?Sized,
{
    fn render_html(&self, f: &mut HtmlFormatter) {
        (**self).render_html(f);
    }

    fn render_url(&self, f: &mut HtmlFormatter) {
        (**self).render_url(f);
    }

    fn render_raw(&self, f: &mut HtmlFormatter) {
        (**self).render_raw(f);
    }
}

impl<T> Render for &mut T
where
    T: Render + ?Sized,
{
    fn render_html(&self, f: &mut HtmlFormatter) {
        (**self).render_html(f);
    }

    fn render_url(&self, f: &mut HtmlFormatter) {
        (**self).render_url(f);
    }

    fn render_raw(&self, f: &mut HtmlFormatter) {
        (**self).render_raw(f);
    }
}

impl<T> Render for Box<T>
where
    T: Render + ?Sized,
{
    fn render_html(&self, f: &mut HtmlFormatter) {
        (**self).render_html(f);
    }

    fn render_url(&self, f: &mut HtmlFormatter) {
        (**self).render_url(f);
    }

    fn render_raw(&self, f: &mut HtmlFormatter) {
        (**self).render_raw(f);
    }
}

impl<T> Render for Option<T>
where
    T: Render,
{
    fn render_html(&self, f: &mut HtmlFormatter) {
        if let Some(this) = self {
            this.render_html(f)
        }
    }

    fn render_url(&self, f: &mut HtmlFormatter) {
        if let Some(this) = self {
            this.render_url(f)
        }
    }

    fn render_raw(&self, f: &mut HtmlFormatter) {
        if let Some(this) = self {
            this.render_raw(f)
        }
    }
}
