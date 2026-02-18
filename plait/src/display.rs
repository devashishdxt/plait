use core::fmt::{self, Write};

use crate::{MaybeAttributeValue, url::is_url_safe};

struct EscapeAdapter<'a>(&'a mut (dyn Write + 'a));

impl<'a> Write for EscapeAdapter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        hescape::escape_to(&mut self.0, s)
    }
}

struct RawAdapter<'a>(&'a mut (dyn Write + 'a));

impl<'a> Write for RawAdapter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}

/// A trait for types that can write themselves as HTML into a [`fmt::Write`] sink.
///
/// `HtmlDisplay` is the HTML counterpart of [`Display`](core::fmt::Display). Where `Display` formats plain text,
/// `HtmlDisplay` writes raw HTML - the output is **not** double-escaped.
///
/// The [`html!`](crate::html) macro returns a value that implements `HtmlDisplay` (via
/// [`HtmlFragment`](crate::HtmlFragment)). You can accept it as a component prop to embed arbitrary HTML content:
///
/// ```rust
/// use plait::{HtmlDisplay, component, html};
///
/// component! {
///     fn Card(title: impl HtmlDisplay) {
///         div(class: "card") {
///             h1 { @(title) }
///             #children
///         }
///     }
/// }
///
/// let html = html! {
///     @Card(title: html! { span { "Rich Title" } }) {
///         p { "body" }
///     }
/// };
///
/// assert_eq!(
///     html.to_string(),
///     "<div class=\"card\"><h1><span>Rich Title</span></h1><p>body</p></div>",
/// );
/// ```
pub trait HtmlDisplay {
    /// Writes this value's HTML representation into `w`.
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result;
}

impl<T> HtmlDisplay for &T
where
    T: HtmlDisplay,
{
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        (**self).html_fmt(w)
    }
}

impl HtmlDisplay for &str {
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        write!(EscapeAdapter(w), "{}", self)
    }
}

pub struct OpenStartTag {
    pub name: &'static str,
}

impl HtmlDisplay for OpenStartTag {
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        write!(RawAdapter(w), "<{}", self.name)
    }
}

pub struct CloseStartTag;

impl HtmlDisplay for CloseStartTag {
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        write!(RawAdapter(w), ">")
    }
}

pub struct CloseTag {
    pub name: &'static str,
}

impl HtmlDisplay for CloseTag {
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        write!(RawAdapter(w), "</{}>", self.name)
    }
}

pub struct Text<T>(pub T);

impl<T> HtmlDisplay for Text<T>
where
    T: fmt::Display,
{
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        write!(EscapeAdapter(w), "{}", self.0)
    }
}

pub struct Raw<T>(pub T);

impl<T> HtmlDisplay for Raw<T>
where
    T: fmt::Display,
{
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        write!(RawAdapter(w), "{}", self.0)
    }
}

pub struct Attribute<T> {
    pub name: &'static str,
    pub value: T,
}

impl<T> HtmlDisplay for Attribute<T>
where
    T: fmt::Display,
{
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        write!(RawAdapter(w), " {}=\"", self.name)?;
        write!(EscapeAdapter(w), "{}", self.value)?;
        write!(RawAdapter(w), "\"")
    }
}

pub struct RawAttribute<T> {
    pub name: &'static str,
    pub value: T,
}

impl<T> HtmlDisplay for RawAttribute<T>
where
    T: fmt::Display,
{
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        write!(RawAdapter(w), " {}=\"{}\"", self.name, self.value)
    }
}

pub struct UrlAttribute<'a> {
    pub name: &'static str,
    pub value: &'a str,
}

impl HtmlDisplay for UrlAttribute<'_> {
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        if is_url_safe(self.value) {
            write!(RawAdapter(w), " {}=\"", self.name)?;
            write!(EscapeAdapter(w), "{}", self.value)?;
            write!(RawAdapter(w), "\"")
        } else {
            Ok(())
        }
    }
}

pub struct MaybeAttribute<T> {
    pub name: &'static str,
    pub value: T,
}

impl<T> HtmlDisplay for MaybeAttribute<T>
where
    T: MaybeAttributeValue,
{
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        if self.value.should_write() {
            write!(RawAdapter(w), " {}", self.name)?;

            if self.value.has_value() {
                write!(RawAdapter(w), "=\"")?;
                self.value.write(&mut EscapeAdapter(w))?;
                write!(RawAdapter(w), "\"")?;
            }
        }

        Ok(())
    }
}

pub struct RawMaybeAttribute<T> {
    pub name: &'static str,
    pub value: T,
}

impl<T> HtmlDisplay for RawMaybeAttribute<T>
where
    T: MaybeAttributeValue,
{
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        if self.value.should_write() {
            write!(RawAdapter(w), " {}", self.name)?;

            if self.value.has_value() {
                write!(RawAdapter(w), "=\"")?;
                self.value.write(&mut RawAdapter(w))?;
                write!(RawAdapter(w), "\"")?;
            }
        }

        Ok(())
    }
}

pub struct UrlMaybeAttribute<'a> {
    pub name: &'static str,
    pub value: Option<&'a str>,
}

impl HtmlDisplay for UrlMaybeAttribute<'_> {
    fn html_fmt(&self, w: &mut (dyn Write + '_)) -> fmt::Result {
        match self.value {
            Some(value) => UrlAttribute {
                name: self.name,
                value,
            }
            .html_fmt(w),
            None => Ok(()),
        }
    }
}
