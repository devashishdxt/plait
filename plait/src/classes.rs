use core::fmt;

/// A trait for types that can contribute to a CSS `class` attribute value.
///
/// Types implementing `ClassPart` can be passed to the [`classes!`](crate::classes) macro and will be joined with
/// spaces. Each implementor decides how to write its value and whether it should be skipped entirely.
///
/// # Provided Implementations
///
/// | Type                      | Behaviour                                                           |
/// |---------------------------|---------------------------------------------------------------------|
/// | `&str`                    | Writes the string directly. Skipped when empty.                     |
/// | `Option<T: ClassPart>`    | Delegates to the inner value when `Some`, skipped when `None`.      |
/// | `&T` where `T: ClassPart` | Delegates through the reference.                                    |
/// | [`Classes<T>`]            | A composed group of class parts (see [`classes!`](crate::classes)). |
///
/// # Example
///
/// ```rust
/// use plait::{html, classes};
///
/// let base = "btn";
/// let variant = Some("btn-primary");
/// let disabled: Option<&str> = None;
///
/// let html = html! {
///     button(class: classes!(base, variant, disabled)) { "Click" }
/// };
///
/// assert_eq!(html.to_string(), "<button class=\"btn btn-primary\">Click</button>");
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
        self.is_empty()
    }
}

impl<T> ClassPart for Option<T>
where
    T: ClassPart,
{
    fn write_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Some(value) => value.write_to(f),
            None => Ok(()),
        }
    }

    fn should_skip(&self) -> bool {
        match self {
            Some(value) => value.should_skip(),
            None => true,
        }
    }
}

impl<T> ClassPart for &T
where
    T: ClassPart,
{
    fn write_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (**self).write_to(f)
    }

    fn should_skip(&self) -> bool {
        (**self).should_skip()
    }
}

/// A wrapper that merges multiple [`ClassPart`] values into a single space-separated string.
///
/// `Classes` implements [`Display`](core::fmt::Display) by joining its parts with spaces. Parts that are empty or
/// `None` are automatically skipped. It also implements [`ClassPart`] itself, so `Classes` values can be nested
/// inside other [`classes!`](crate::classes) calls.
///
/// This type is normally created via the [`classes!`](crate::classes) macro rather than constructed directly.
///
/// # Examples
///
/// Nested composition - passing a `Classes` value into another component's [`ClassPart`] prop:
///
/// ```rust
/// use plait::{component, html, classes, ClassPart};
///
/// component! {
///     fn Button(class: impl ClassPart) {
///         button(class: classes!("btn", class)) {}
///     }
/// }
///
/// let html = html! {
///     @Button(class: classes!("btn-secondary", "btn-lg")) {}
/// };
///
/// assert_eq!(html.to_string(), "<button class=\"btn btn-secondary btn-lg\"></button>");
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
/// Accepts any number of expressions that implement [`ClassPart`]. Values are joined with spaces, and empty or `None`
/// values are automatically skipped.
///
/// # Examples
///
/// Basic usage with string literals and optional classes:
///
/// ```rust
/// use plait::{html, classes};
///
/// let is_active = true;
/// let variant = Some("primary");
///
/// let html = html! {
///     div(class: classes!(
///         "btn",
///         if is_active { "active" } else { "" },
///         variant,
///     )) {
///         "Button"
///     }
/// };
///
/// assert_eq!(html.to_string(), "<div class=\"btn active primary\">Button</div>");
/// ```
///
/// Using `classes!` inside a component:
///
/// ```rust
/// use plait::{component, html, classes, ClassPart};
///
/// component! {
///     fn Button(class: impl ClassPart) {
///         button(class: classes!("btn", class), #attrs) {
///             #children
///         }
///     }
/// }
///
/// let html = html! {
///     @Button(class: "btn-primary") { "Submit" }
/// };
///
/// assert_eq!(html.to_string(), "<button class=\"btn btn-primary\">Submit</button>");
/// ```
#[macro_export]
macro_rules! classes {
    ($($class:expr),+ $(,)?) => {
        $crate::Classes(($($class,)+))
    };
}
