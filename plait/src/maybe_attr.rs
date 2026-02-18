use core::fmt;

/// A trait for values that may or may not produce an HTML attribute.
///
/// `MaybeAttributeValue` powers the `?:` (optional attribute) syntax in [`html!`](crate::html). When an attribute is
/// written as `name?: value`, plait calls this trait to decide whether to render the attribute and, if so, whether it
/// carries a value.
///
/// # Provided Implementations
///
/// | Type                                | Behaviour                                                                 |
/// |-------------------------------------|---------------------------------------------------------------------------|
/// | `bool`                              | `true` emits a boolean attribute (`disabled`), `false` omits it entirely. |
/// | `Option<T: Display>`                | `Some(v)` emits `name="v"`, `None` omits the attribute.                   |
/// | `&T` where `T: MaybeAttributeValue` | Delegates through the reference.                                          |
///
/// # Example
///
/// ```rust
/// use plait::html;
///
/// let class = Some("active");
/// let disabled = false;
///
/// let html = html! {
///     button(class?: class, disabled?: disabled) { "Click" }
/// };
///
/// assert_eq!(html.to_string(), "<button class=\"active\">Click</button>");
/// ```
pub trait MaybeAttributeValue {
    fn should_write(&self) -> bool;

    fn has_value(&self) -> bool;

    fn write(&self, w: &mut impl fmt::Write) -> fmt::Result;
}

impl MaybeAttributeValue for bool {
    fn should_write(&self) -> bool {
        *self
    }

    fn has_value(&self) -> bool {
        false
    }

    fn write(&self, _: &mut impl fmt::Write) -> fmt::Result {
        Ok(())
    }
}

impl<T> MaybeAttributeValue for Option<T>
where
    T: fmt::Display,
{
    fn should_write(&self) -> bool {
        self.is_some()
    }

    fn has_value(&self) -> bool {
        self.is_some()
    }

    fn write(&self, w: &mut impl fmt::Write) -> fmt::Result {
        match self {
            Some(value) => write!(w, "{value}"),
            None => Ok(()),
        }
    }
}

impl<T> MaybeAttributeValue for &T
where
    T: MaybeAttributeValue,
{
    fn should_write(&self) -> bool {
        (**self).should_write()
    }

    fn has_value(&self) -> bool {
        (**self).has_value()
    }

    fn write(&self, w: &mut impl fmt::Write) -> fmt::Result {
        (**self).write(w)
    }
}
