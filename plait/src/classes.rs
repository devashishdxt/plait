use core::fmt;

/// A trait for types that can be used as parts of a CSS class attribute.
///
/// This trait enables different types to participate in class merging via the [`classes!`](crate::classes) macro.
/// Implementations handle how each type writes its class value and whether it should be skipped.
///
/// # Provided Implementations
///
/// - `&str`: Writes the string directly. Skipped if the string is empty.
/// - `Option<&str>`: Writes the inner string if `Some`, skipped if `None` or empty.
///
/// # Example
///
/// ```rust
/// use plait::{html, classes, render};
///
/// let base = "btn";
/// let variant = Some("btn-primary");
/// let disabled: Option<&str> = None;
///
/// let html = render(html! {
///     button(class: classes!(base, variant, disabled)) { "Click" }
/// });
///
/// assert_eq!(html, "<button class=\"btn btn-primary\">Click</button>");
/// ```
pub trait ClassPart {
    /// Writes this class part to the formatter.
    fn write_to(&self, f: &mut fmt::Formatter) -> fmt::Result;

    /// Returns `true` if this class part should be skipped when merging.
    fn should_skip(&self) -> bool;
}

impl ClassPart for &str {
    fn write_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }

    fn should_skip(&self) -> bool {
        (**self).is_empty()
    }
}

impl ClassPart for Option<&str> {
    fn write_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }

    fn should_skip(&self) -> bool {
        self.is_none_or(str::is_empty)
    }
}

/// A wrapper that merges multiple class parts into a single space-separated string.
///
/// `Classes` wraps a tuple of [`ClassPart`] values and implements [`Display`](core::fmt::Display) to join them with
/// spaces. Empty or `None` values are automatically skipped.
///
/// This type is typically created via the [`classes!`](crate::classes) macro rather than directly.
///
/// # Example
///
/// ```rust
/// use plait::Classes;
///
/// let classes = Classes(("btn", Some("btn-primary"), None::<&str>));
/// assert_eq!(classes.to_string(), "btn btn-primary");
/// ```
pub struct Classes<T>(pub T);

impl<T: ClassPart> fmt::Display for Classes<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.should_skip() {
            Ok(())
        } else {
            self.0.write_to(f)
        }
    }
}

impl<T: ClassPart> ClassPart for Classes<T> {
    fn write_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ClassPart::write_to(&self.0, f)
    }

    fn should_skip(&self) -> bool {
        ClassPart::should_skip(&self.0)
    }
}

macro_rules! impl_classes_traits {
    ($($idx:tt: $T:ident),+) => {
        impl<$($T: $crate::ClassPart),+> ::core::fmt::Display for $crate::Classes<($($T,)+)> {
            #[allow(unused_assignments)]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut need_space = false;
                $(
                    if !$crate::ClassPart::should_skip(&self.0.$idx) {
                        if need_space {
                            ::core::fmt::Write::write_char(f, ' ')?;
                        }
                        $crate::ClassPart::write_to(&self.0.$idx, f)?;
                        need_space = true;
                    }
                )+
                Ok(())
            }
        }

        impl<$($T: $crate::ClassPart),+> $crate::ClassPart for $crate::Classes<($($T,)+)> {
            #[allow(unused_assignments)]
            fn write_to(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self, f)
            }

            fn should_skip(&self) -> bool {
                true $( && $crate::ClassPart::should_skip(&self.0.$idx) )+
            }
        }
    };
  }

impl_classes_traits!(0: T0);
impl_classes_traits!(0: T0, 1: T1);
impl_classes_traits!(0: T0, 1: T1, 2: T2);
impl_classes_traits!(0: T0, 1: T1, 2: T2, 3: T3);
impl_classes_traits!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4);
impl_classes_traits!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5);
impl_classes_traits!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6);
impl_classes_traits!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7);

/// Merges multiple CSS class values into a single space-separated string.
///
/// This macro accepts any number of expressions that implement [`ClassPart`]. Values are joined with spaces, and
/// empty or `None` values are automatically skipped.
///
/// # Examples
///
/// Basic usage with string literals and optional classes:
///
/// ```rust
/// use plait::{html, classes, render};
///
/// let is_active = true;
/// let variant = Some("primary");
///
/// let html = render(html! {
///     div(class: classes!(
///         "btn",
///         if is_active { "active" } else { "" },
///         variant,
///     )) {
///         "Button"
///     }
/// });
///
/// assert_eq!(html, "<div class=\"btn active primary\">Button</div>");
/// ```
///
/// Using `classes!` inside a component:
///
/// ```rust
/// use plait::{component, html, classes, render};
///
/// component! {
///     fn Button(variant: Option<&str>) {
///         button(class: classes!("btn", variant), #attrs) {
///             #children
///         }
///     }
/// }
///
/// let html = render(html! {
///     @Button(variant: Some("btn-primary")) { "Submit" }
/// });
///
/// assert_eq!(html, "<button class=\"btn btn-primary\">Submit</button>");
/// ```
#[macro_export]
macro_rules! classes {
    ($($class:expr),+ $(,)?) => {
        $crate::Classes(($($class,)+))
    };
}
