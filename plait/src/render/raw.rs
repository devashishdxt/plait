use std::{borrow::Cow, fmt};

/// Trait for types that can be rendered as raw (unescaped) text.
///
/// When a value is embedded in an [`html!`](crate::html) template with `#(expr)`, it is rendered through this trait
/// **without** HTML escaping. Use this when the value is already known to be safe HTML.
///
/// # Built-in implementations
///
/// The same types that implement [`RenderEscaped`](crate::RenderEscaped) also implement `RenderRaw`. For `&str` and
/// `String`, the output is written verbatim (no escaping). Numeric and boolean types produce the same output as their
/// escaped counterparts since they contain no HTML-special characters.
pub trait RenderRaw {
    /// Writes the raw (unescaped) representation of `self` into `f`.
    fn render_raw(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result;
}

impl<T> RenderRaw for &T
where
    T: RenderRaw + ?Sized,
{
    #[inline]
    fn render_raw(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        (**self).render_raw(f)
    }
}

impl RenderRaw for str {
    #[inline]
    fn render_raw(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        f.write_str(self)
    }
}

impl RenderRaw for String {
    #[inline]
    fn render_raw(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        f.write_str(self)
    }
}

impl RenderRaw for bool {
    #[inline]
    fn render_raw(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        f.write_str(if *self { "true" } else { "false" })
    }
}

impl<T> RenderRaw for Option<T>
where
    T: RenderRaw,
{
    #[inline]
    fn render_raw(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        match self {
            Some(value) => value.render_raw(f),
            None => Ok(()),
        }
    }
}

impl<'a, T> RenderRaw for Cow<'a, T>
where
    T: RenderRaw + ToOwned + ?Sized + 'a,
{
    #[inline]
    fn render_raw(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        self.as_ref().render_raw(f)
    }
}

macro_rules! impl_render_raw_itoa {
    ($ty:ty) => {
        impl RenderRaw for $ty {
            #[inline]
            fn render_raw(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
                let mut buffer = itoa::Buffer::new();
                f.write_str(buffer.format(*self))
            }
        }
    };
}

impl_render_raw_itoa!(usize);
impl_render_raw_itoa!(isize);
impl_render_raw_itoa!(u8);
impl_render_raw_itoa!(u16);
impl_render_raw_itoa!(u32);
impl_render_raw_itoa!(u64);
impl_render_raw_itoa!(u128);
impl_render_raw_itoa!(i8);
impl_render_raw_itoa!(i16);
impl_render_raw_itoa!(i32);
impl_render_raw_itoa!(i64);
impl_render_raw_itoa!(i128);

macro_rules! impl_render_raw_ryu {
    ($ty:ty) => {
        impl RenderRaw for $ty {
            #[inline]
            fn render_raw(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
                let mut buffer = ryu::Buffer::new();
                f.write_str(buffer.format(*self))
            }
        }
    };
}

impl_render_raw_ryu!(f32);
impl_render_raw_ryu!(f64);
