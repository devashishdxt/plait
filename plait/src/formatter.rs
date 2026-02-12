use core::fmt::{self, Display, Write};

use crate::{
    Html, IntoHtml, IntoHtmlRaw, MaybeAttr,
    url::{is_url_attribute, is_url_safe},
};

/// A formatter for building HTML content safely and efficiently.
///
/// `HtmlFormatter` provides methods for constructing HTML by writing tags, attributes, and content to an underlying
/// [`Html`](crate::Html) buffer. It handles HTML escaping automatically when using the `*_escaped` methods, helping
/// prevent XSS vulnerabilities.
///
/// # Note
///
/// This type should not be used directly. Instead, use the [`html!`](crate::html!) macro with
/// [`render`](crate::render) or [`render_with_capacity`](crate::render_with_capacity) functions.
///
/// # Usage
///
/// ```rust
/// use plait::{Html, HtmlFormatter};
///
/// let mut html = Html::new();
/// let mut f = HtmlFormatter::new(&mut html);
///
/// f.open_tag("div");
/// f.write_attribute_escaped("class", "container");
/// f.close_start_tag();
/// f.write_html_escaped("<script>alert('xss')</script>");
/// f.close_tag("div");
///
/// assert_eq!(html, "<div class=\"container\">&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;</div>");
/// ```
pub struct HtmlFormatter<'a>(&'a mut Html);

impl<'a> HtmlFormatter<'a> {
    /// Create a new `HtmlFormatter` for the given `Html` string.
    pub fn new(html: &'a mut Html) -> Self {
        HtmlFormatter(html)
    }

    /// Returns a raw writer for current `HtmlFormatter`.
    pub(crate) fn raw_writer(&mut self) -> &mut String {
        self.0.inner_mut()
    }

    /// Returns a writer that escapes HTML special characters for current `HtmlFormatter`.
    pub(crate) fn html_escaped_writer(&mut self) -> HtmlEscapedWriter<'_> {
        HtmlEscapedWriter(self.0)
    }

    /// Opens a tag with the given name.
    pub fn open_tag(&mut self, tag_name: &str) {
        write!(self.raw_writer(), "<{tag_name}").unwrap();
    }

    /// Closes start tag (should be called after writing attributes).
    pub fn close_start_tag(&mut self) {
        self.raw_writer().push('>');
    }

    /// Closes a tag with the given name.
    pub fn close_tag(&mut self, tag_name: &str) {
        if !is_void_element(tag_name) {
            write!(self.raw_writer(), "</{tag_name}>").unwrap();
        }
    }

    /// Write HTML content to the formatter without escaping any special HTML characters.
    pub fn write_raw(&mut self, raw: impl IntoHtmlRaw) {
        raw.render_raw_to(self);
    }

    /// Write HTML content to the formatter, escaping any special HTML characters.
    pub fn write_html_escaped(&mut self, html: impl IntoHtml) {
        html.render_to(self);
    }

    /// Write an attribute to the formatter without escaping any special HTML characters.
    pub fn write_attribute_raw(&mut self, name: &str, value: impl Display) {
        write!(self.raw_writer(), " {name}=\"").unwrap();
        write!(self.raw_writer(), "{value}").unwrap();
        self.raw_writer().push('"');
    }

    /// Write an attribute to the formatter, escaping any special HTML characters in value.
    pub fn write_attribute_escaped(&mut self, name: &str, value: impl Display) {
        write!(self.raw_writer(), " {name}=\"").unwrap();
        write!(self.html_escaped_writer(), "{value}").unwrap();
        self.raw_writer().push('"');
    }

    /// Write an optional or boolean attribute to the formatter without escaping any special HTML characters.
    pub fn write_maybe_attribute_raw(&mut self, name: &str, value: impl MaybeAttr) {
        value.write_raw(self, name);
    }

    /// Write an optional or boolean attribute to the formatter, escaping any special HTML characters in value.
    pub fn write_maybe_attribute_escaped(&mut self, name: &str, value: impl MaybeAttr) {
        value.write_escaped(self, name);
    }

    /// Write an optional attribute to the formatter without escaping any special HTML characters.
    pub fn write_optional_attribute_raw(&mut self, name: &str, value: Option<impl Display>) {
        if let Some(value) = value {
            self.write_attribute_raw(name, value);
        }
    }

    /// Write an optional attribute to the formatter, escaping any special HTML characters in value.
    pub fn write_optional_attribute_escaped(&mut self, name: &str, value: Option<impl Display>) {
        if let Some(value) = value {
            self.write_attribute_escaped(name, value);
        }
    }

    /// Write an URL attribute to the formatter, escaping any special HTML characters in value.
    pub fn write_url_attribute_escaped(&mut self, name: &str, value: &str) {
        if is_url_attribute(name) && is_url_safe(value) {
            write!(self.raw_writer(), " {name}=\"").unwrap();
            write!(self.html_escaped_writer(), "{value}").unwrap();
            self.raw_writer().push('"');
        }
    }

    /// Write an optional URL attribute to the formatter, escaping any special HTML characters in value.
    pub fn write_optional_url_attribute_escaped(&mut self, name: &str, value: Option<&str>) {
        if let Some(value) = value {
            self.write_url_attribute_escaped(name, value);
        }
    }

    /// Write a boolean attribute to the formatter.
    pub fn write_boolean_attribute(&mut self, name: &str, value: bool) {
        if value {
            write!(self.raw_writer(), " {name}").unwrap()
        }
    }
}

/// Returns true if the given element name is a void element.
/// Expects the name to be in ASCII lowercase.
fn is_void_element(name: &str) -> bool {
    matches!(
        name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

/// A writer that escapes HTML special characters.
pub(crate) struct HtmlEscapedWriter<'a>(&'a mut Html);

impl Write for HtmlEscapedWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        hescape::escape_to(self.0.inner_mut(), s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_formatter_open_tag() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.open_tag("div");

        assert_eq!(html, "<div");
    }

    #[test]
    fn test_html_formatter_close_start_tag() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.close_start_tag();

        assert_eq!(html, ">");
    }

    #[test]
    fn test_html_formatter_close_tag() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.close_tag("div");
        f.close_tag("br"); // Not written (void element)

        assert_eq!(html, "</div>");
    }

    #[test]
    fn test_html_formatter_write_raw() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.write_raw("<div>Hello World!</div>");

        assert_eq!(html, "<div>Hello World!</div>");
    }

    #[test]
    fn test_html_formatter_write_html_escaped() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.write_html_escaped("<div>Hello World!</div>");

        assert_eq!(html, "&lt;div&gt;Hello World!&lt;/div&gt;");
    }

    #[test]
    fn test_html_formatter_write_attribute_raw() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.write_attribute_raw("class", "<strong>Hello</strong>");

        assert_eq!(html, " class=\"<strong>Hello</strong>\"");
    }

    #[test]
    fn test_html_formatter_write_attribute_escaped() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.write_attribute_escaped("class", "<strong>\"Hello\"</strong>");

        assert_eq!(
            html,
            " class=\"&lt;strong&gt;&quot;Hello&quot;&lt;/strong&gt;\""
        );
    }

    #[test]
    fn test_html_formatter_write_maybe_attribute_raw() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.write_maybe_attribute_raw("class", Some("<strong>Hello</strong>"));
        f.write_maybe_attribute_raw("id", None::<&str>);
        f.write_maybe_attribute_raw("checked", true);
        f.write_maybe_attribute_raw("disabled", false);

        assert_eq!(html, " class=\"<strong>Hello</strong>\" checked");
    }

    #[test]
    fn test_html_formatter_write_maybe_attribute_escaped() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.write_maybe_attribute_escaped("class", Some("<strong>Hello</strong>"));
        f.write_maybe_attribute_escaped("id", None::<&str>);
        f.write_maybe_attribute_escaped("checked", true);
        f.write_maybe_attribute_escaped("disabled", false);

        assert_eq!(
            html,
            " class=\"&lt;strong&gt;Hello&lt;/strong&gt;\" checked"
        );
    }

    #[test]
    fn test_html_formatter_write_optional_attribute_raw() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.write_optional_attribute_raw("class", Some("<strong>Hello</strong>"));
        f.write_optional_attribute_raw("id", None::<&str>);

        assert_eq!(html, " class=\"<strong>Hello</strong>\"");
    }

    #[test]
    fn test_html_formatter_write_optional_attribute_escaped() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.write_optional_attribute_escaped("class", Some("<strong>\"Hello\"</strong>"));
        f.write_optional_attribute_escaped("id", None::<&str>);

        assert_eq!(
            html,
            " class=\"&lt;strong&gt;&quot;Hello&quot;&lt;/strong&gt;\""
        );
    }

    #[test]
    fn test_html_formatter_write_url_attribute_escaped() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.write_url_attribute_escaped("href", "https://example.com?q=\"hello\"");
        f.write_url_attribute_escaped("class", "btn"); // Not written (not a url attribute)
        f.write_url_attribute_escaped("src", "javascript:alert('XSS')"); // Not written (unsafe url)

        assert_eq!(html, " href=\"https://example.com?q=&quot;hello&quot;\"");
    }

    #[test]
    fn test_html_formatter_write_optional_url_attribute_escaped() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.write_optional_url_attribute_escaped("href", Some("https://example.com?q=\"hello\""));
        f.write_optional_url_attribute_escaped("src", None::<&str>);

        assert_eq!(html, " href=\"https://example.com?q=&quot;hello&quot;\"");
    }

    #[test]
    fn test_html_formatter_write_boolean_attribute() {
        let mut html = Html::new();
        let mut f = HtmlFormatter::new(&mut html);

        f.write_boolean_attribute("checked", true);
        f.write_boolean_attribute("active", false);

        assert_eq!(html, " checked");
    }
}
