use std::{borrow::Cow, fmt};

use crate::utils::escape_html_to;

pub trait RenderEscaped {
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
        impl RenderEscaped for $ty {
            #[inline]
            fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
                let mut buffer = itoa::Buffer::new();
                f.write_str(buffer.format(*self))
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
        impl RenderEscaped for $ty {
            #[inline]
            fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
                let mut buffer = ryu::Buffer::new();
                f.write_str(buffer.format(*self))
            }
        }
    };
}

impl_render_escaped_ryu!(f32);
impl_render_escaped_ryu!(f64);
