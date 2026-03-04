use std::{borrow::Cow, fmt};

use crate::utils::escape_html_to;

/// Trait for types that can be rendered as HTML-escaped text.
///
/// When a value is embedded in an [`html!`](crate::html) template with `(expr)`, it is rendered through this trait,
/// which ensures HTML-special characters (`&`, `<`, `>`, `"`, `'`) are escaped.
///
/// # Built-in implementations
///
/// | Type                                                       | Behavior                                   |
/// |------------------------------------------------------------|--------------------------------------------|
/// | `&str`, `String`                                           | HTML-escaped output                        |
/// | `bool`                                                     | `"true"` or `"false"`                      |
/// | `Option<T: RenderEscaped>`                                 | Renders inner value, or nothing for `None` |
/// | `Cow<'_, T: RenderEscaped>`                                | Delegates to inner value                   |
/// | Integer types (`u8`–`u128`, `i8`–`i128`, `usize`, `isize`) | Formatted via [`itoa`]                     |
/// | Float types (`f32`, `f64`)                                 | Formatted via [`ryu`]                      |
/// | [`Html`](crate::Html)                                      | Written as-is (already escaped)            |
/// | [`HtmlFragment`](crate::HtmlFragment)                      | Renders the fragment                       |
/// | `&T` where `T: RenderEscaped`                              | Delegates to inner value                   |
pub trait RenderEscaped {
    /// Writes the HTML-escaped representation of `self` into `f`.
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result;
}

impl<T> RenderEscaped for &T
where
    T: RenderEscaped + ?Sized,
{
    #[inline]
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        (**self).render_escaped(f)
    }
}

impl RenderEscaped for str {
    #[inline]
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        escape_html_to(f, self)
    }
}

impl RenderEscaped for String {
    #[inline]
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        escape_html_to(f, self)
    }
}

impl RenderEscaped for bool {
    #[inline]
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        f.write_str(if *self { "true" } else { "false" })
    }
}

impl<T> RenderEscaped for Option<T>
where
    T: RenderEscaped,
{
    #[inline]
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        match self {
            Some(value) => value.render_escaped(f),
            None => Ok(()),
        }
    }
}

impl<'a, T> RenderEscaped for Cow<'a, T>
where
    T: RenderEscaped + ToOwned + ?Sized + 'a,
{
    #[inline]
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        self.as_ref().render_escaped(f)
    }
}

macro_rules! impl_render_escaped_itoa {
    ($ty:ty) => {
        #[cfg(feature = "itoa")]
        impl RenderEscaped for $ty {
            #[inline]
            fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
                let mut buffer = itoa::Buffer::new();
                f.write_str(buffer.format(*self))
            }
        }

        #[cfg(not(feature = "itoa"))]
        impl RenderEscaped for $ty {
            #[inline]
            fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
                write!(f, "{}", *self)
            }
        }
    };
}

impl_render_escaped_itoa!(usize);
impl_render_escaped_itoa!(isize);
impl_render_escaped_itoa!(u8);
impl_render_escaped_itoa!(u16);
impl_render_escaped_itoa!(u32);
impl_render_escaped_itoa!(u64);
impl_render_escaped_itoa!(u128);
impl_render_escaped_itoa!(i8);
impl_render_escaped_itoa!(i16);
impl_render_escaped_itoa!(i32);
impl_render_escaped_itoa!(i64);
impl_render_escaped_itoa!(i128);

macro_rules! impl_render_escaped_ryu {
    ($ty:ty) => {
        #[cfg(feature = "ryu")]
        impl RenderEscaped for $ty {
            #[inline]
            fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
                let mut buffer = ryu::Buffer::new();
                f.write_str(buffer.format(*self))
            }
        }

        #[cfg(not(feature = "ryu"))]
        impl RenderEscaped for $ty {
            #[inline]
            fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
                write!(f, "{}", *self)
            }
        }
    };
}

impl_render_escaped_ryu!(f32);
impl_render_escaped_ryu!(f64);
