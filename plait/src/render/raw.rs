use std::{borrow::Cow, fmt};

pub trait RenderRaw {
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

macro_rules! impl_render_escaped_itoa {
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
        impl RenderRaw for $ty {
            #[inline]
            fn render_raw(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
                let mut buffer = ryu::Buffer::new();
                f.write_str(buffer.format(*self))
            }
        }
    };
}

impl_render_escaped_ryu!(f32);
impl_render_escaped_ryu!(f64);
