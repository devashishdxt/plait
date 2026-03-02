use std::fmt;

use crate::{RenderEscaped, RenderRaw};

/// Trait for conditionally rendering an HTML attribute with a raw (unescaped) value.
///
/// Used by the `attr?: #(expr)` syntax in [`html!`](crate::html). The attribute is only rendered when the value is
/// "present" (e.g. `Some(_)` or `true`).
///
/// # Built-in implementations
///
/// | Type                   | Behavior                                                            |
/// |------------------------|---------------------------------------------------------------------|
/// | `bool`                 | Renders the attribute name (no value) if `true`; nothing if `false` |
/// | `Option<T: RenderRaw>` | Renders `name="value"` if `Some`; nothing if `None`                 |
pub trait RenderMaybeAttributeRaw {
    /// Conditionally writes ` name` or ` name="value"` into `f`.
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

/// Trait for conditionally rendering an HTML attribute with an escaped value.
///
/// Used by the `attr?: expr` syntax in [`html!`](crate::html). The attribute is only rendered when the value is
/// "present" (e.g. `Some(_)` or `true`).
///
/// # Built-in implementations
///
/// | Type                       | Behavior                                                            |
/// |----------------------------|---------------------------------------------------------------------|
/// | `bool`                     | Renders the attribute name (no value) if `true`; nothing if `false` |
/// | `Option<T: RenderEscaped>` | Renders `name="value"` (escaped) if `Some`; nothing if `None`       |
pub trait RenderMaybeAttributeEscaped {
    /// Conditionally writes ` name` or ` name="value"` (escaped) into `f`.
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
