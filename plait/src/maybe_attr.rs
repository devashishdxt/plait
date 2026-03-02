use std::fmt;

use crate::{RenderEscaped, RenderRaw};

pub trait RenderMaybeAttributeRaw {
    fn render_maybe_attribute_raw(&self, name: &str, f: &mut (dyn fmt::Write + '_)) -> fmt::Result;
}

impl<T> RenderMaybeAttributeRaw for &T
where
    T: RenderMaybeAttributeRaw + ?Sized,
{
    fn render_maybe_attribute_raw(&self, name: &str, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        (**self).render_maybe_attribute_raw(name, f)
    }
}

impl RenderMaybeAttributeRaw for bool {
    fn render_maybe_attribute_raw(&self, name: &str, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        if *self {
            f.write_str(" ")?;
            f.write_str(name)?;

            Ok(())
        } else {
            Ok(())
        }
    }
}

impl<T> RenderMaybeAttributeRaw for Option<T>
where
    T: RenderRaw,
{
    fn render_maybe_attribute_raw(&self, name: &str, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        match self {
            Some(value) => {
                f.write_str(" ")?;
                f.write_str(name)?;
                f.write_str("=\"")?;
                value.render_raw(f)?;
                f.write_str("\"")?;

                Ok(())
            }
            None => Ok(()),
        }
    }
}

pub trait RenderMaybeAttributeEscaped {
    fn render_maybe_attribute_escaped(
        &self,
        name: &str,
        f: &mut (dyn fmt::Write + '_),
    ) -> fmt::Result;
}

impl<T> RenderMaybeAttributeEscaped for &T
where
    T: RenderMaybeAttributeEscaped + ?Sized,
{
    fn render_maybe_attribute_escaped(
        &self,
        name: &str,
        f: &mut (dyn fmt::Write + '_),
    ) -> fmt::Result {
        (**self).render_maybe_attribute_escaped(name, f)
    }
}

impl RenderMaybeAttributeEscaped for bool {
    fn render_maybe_attribute_escaped(
        &self,
        name: &str,
        f: &mut (dyn fmt::Write + '_),
    ) -> fmt::Result {
        if *self {
            f.write_str(" ")?;
            f.write_str(name)?;

            Ok(())
        } else {
            Ok(())
        }
    }
}

impl<T> RenderMaybeAttributeEscaped for Option<T>
where
    T: RenderEscaped,
{
    fn render_maybe_attribute_escaped(
        &self,
        name: &str,
        f: &mut (dyn fmt::Write + '_),
    ) -> fmt::Result {
        match self {
            Some(value) => {
                f.write_str(" ")?;
                f.write_str(name)?;
                f.write_str("=\"")?;
                value.render_escaped(f)?;
                f.write_str("\"")?;

                Ok(())
            }
            None => Ok(()),
        }
    }
}
