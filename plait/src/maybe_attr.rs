use core::fmt::Display;

use crate::HtmlFormatter;

/// A trait for values that can be conditionally written as HTML attributes.
///
/// This trait provides a unified interface for handling HTML attributes that may or may not be rendered, depending on
/// their value. It supports two main use cases:
///
/// - **Boolean attributes** (e.g., `disabled`, `checked`, `readonly`): These are rendered without a value when
///   `true`, and omitted entirely when `false`.
/// - **Optional attributes**: These are rendered with their value when `Some`, and omitted when `None`.
///
/// # Implementations
///
/// - `bool`: For boolean HTML attributes. When `true`, renders as ` name` (no value). When `false`, renders nothing.
/// - `Option<T>` where `T: Display`: For optional attributes. When `Some(value)`, renders as ` name="value"`. When
///   `None`, renders nothing.
pub trait MaybeAttr {
    /// Writes the attribute with HTML escaping applied to the value.
    fn write_escaped(self, f: &mut HtmlFormatter<'_>, name: &str);

    /// Writes the attribute without HTML escaping.
    fn write_raw(self, f: &mut HtmlFormatter<'_>, name: &str);
}

impl MaybeAttr for bool {
    fn write_escaped(self, f: &mut HtmlFormatter<'_>, name: &str) {
        f.write_boolean_attribute(name, self);
    }

    fn write_raw(self, f: &mut HtmlFormatter<'_>, name: &str) {
        f.write_boolean_attribute(name, self);
    }
}

impl<T> MaybeAttr for Option<T>
where
    T: Display,
{
    fn write_escaped(self, f: &mut HtmlFormatter<'_>, name: &str) {
        if let Some(value) = self {
            f.write_attribute_escaped(name, value);
        }
    }

    fn write_raw(self, f: &mut HtmlFormatter<'_>, name: &str) {
        if let Some(value) = self {
            f.write_attribute_raw(name, value);
        }
    }
}
