use std::fmt;

use crate::RenderEscaped;

/// Trait for values that can be used as CSS class names in the [`classes!`](crate::classes) macro.
///
/// Implementors define whether the class should be skipped (e.g. empty string or `None`) and how to render the class
/// name. Multiple `Class` values are joined with spaces.
///
/// # Built-in implementations
///
/// | Type                                 | Behavior                                              |
/// |--------------------------------------|-------------------------------------------------------|
/// | `&str`                               | Skipped if empty; otherwise HTML-escaped              |
/// | `Option<T: Class>`                   | Skipped if `None`; otherwise delegates to inner value |
/// | `&T` where `T: Class`                | Delegates to inner value                              |
/// | Tuples of `Class` (up to 8 elements) | Renders non-skipped elements separated by spaces      |
/// | `Classes<T: Class>`                  | Renders non-skipped elements separated by spaces      |
pub trait Class {
    /// Returns `true` if this class should be omitted from the output.
    fn should_skip(&self) -> bool;

    /// Writes the class name(s) into `f`.
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result;
}

impl<T> Class for &T
where
    T: Class + ?Sized,
{
    fn should_skip(&self) -> bool {
        (**self).should_skip()
    }

    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        (**self).render_escaped(f)
    }
}

impl<T> Class for Option<T>
where
    T: Class,
{
    fn should_skip(&self) -> bool {
        match self {
            Some(value) => value.should_skip(),
            None => true,
        }
    }

    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        match self {
            Some(value) => value.render_escaped(f),
            None => Ok(()),
        }
    }
}

impl Class for str {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }

    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        RenderEscaped::render_escaped(self, f)
    }
}

/// A wrapper that turns a tuple of [`Class`] values into a single renderable class string.
///
/// You typically create this via the [`classes!`](crate::classes) macro rather than constructing it directly:
///
/// ```
/// use plait::{classes, html, ToHtml};
///
/// let extra: Option<&str> = Some("highlighted");
///
/// let frag = html! {
///     div(class: classes!("base", "primary", extra)) {}
/// };
/// assert_eq!(frag.to_html(), r#"<div class="base primary highlighted"></div>"#);
/// ```
///
/// `Classes<T>` implements [`RenderEscaped`] and [`Display`](std::fmt::Display), so it can be used anywhere a
/// renderable value is expected.
pub struct Classes<T>(pub T);

impl<T> Class for Classes<T>
where
    T: Class,
{
    fn should_skip(&self) -> bool {
        self.0.should_skip()
    }

    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        self.0.render_escaped(f)
    }
}

impl<T> RenderEscaped for Classes<T>
where
    T: Class,
{
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        Class::render_escaped(&self.0, f)
    }
}

impl<T> fmt::Display for Classes<T>
where
    T: Class,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Class::render_escaped(&self, f)
    }
}

macro_rules! impl_class_for_tuple {
    ($($idx:tt: $T:ident),+) => {
        impl<$($T: $crate::Class),+> $crate::Class for ($($T,)+) {
            fn should_skip(&self) -> bool {
                true $( && $crate::Class::should_skip(&self.$idx) )+
            }

            #[allow(unused_assignments)]
            fn render_escaped(&self, f: &mut (dyn ::core::fmt::Write + '_)) -> ::core::fmt::Result {
                let mut needs_space = false;

                $(
                    if !$crate::Class::should_skip(&self.$idx) {
                        if needs_space {
                            ::core::fmt::Write::write_char(f, ' ')?;
                        }
                        $crate::Class::render_escaped(&self.$idx, f)?;
                        needs_space = true;
                    }
                )+

                Ok(())
            }
        }
    };
}

impl_class_for_tuple!(0: T0);
impl_class_for_tuple!(0: T0, 1: T1);
impl_class_for_tuple!(0: T0, 1: T1, 2: T2);
impl_class_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3);
impl_class_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4);
impl_class_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5);
impl_class_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6);
impl_class_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7);

/// Combines multiple CSS class values into a single [`Classes`] value.
///
/// Empty strings and `None` values are automatically skipped. Non-skipped values are
/// separated by spaces.
///
/// Each argument must implement the [`Class`] trait.
///
/// # Example
///
/// ```
/// use plait::{classes, html, ToHtml};
///
/// let active: Option<&str> = Some("active");
/// let hidden: Option<&str> = None;
///
/// let frag = html! {
///     div(class: classes!("btn", "btn-primary", active, hidden)) {}
/// };
/// assert_eq!(frag.to_html(), r#"<div class="btn btn-primary active"></div>"#);
/// ```
#[macro_export]
macro_rules! classes {
    ($($class:expr),+ $(,)?) => {
        $crate::Classes(($($class,)+))
    };
}
