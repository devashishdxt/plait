use crate::{
    Attributes, Error, EscapeMode, Html, Render,
    escape::{escape_html, escape_url, resolve_escape_mode_for_element},
};

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

/// The current state of the formatter state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum FormatterState {
    /// No element is currently being written. Ready to start a new element or write raw content.
    #[default]
    Idle,

    /// An opening tag has started (e.g., `<div`). Attributes can be added. The `>` has not been written yet.
    TagOpened,

    /// The opening tag is closed (`>`). Content or child elements can be written.
    InContent,
}

/// An entry in the element stack, tracking the element name and whether it's a void element.
struct ElementEntry {
    name: &'static str,
    is_void: bool,
    attributes: Attributes,
}

/// A stateful formatter for constructing well-formed HTML output.
///
/// `HtmlFormatter` uses a state machine to ensure HTML is generated correctly:
/// - Elements are properly opened and closed
/// - Attributes are only written when a tag is open
/// - Void elements (like `<br>`, `<img>`) are handled correctly
/// - Content is properly escaped based on context
///
/// # State Machine
///
/// The formatter transitions between three states:
/// - **Idle**: Ready to start a new element or write raw content
/// - **TagOpened**: An opening tag has started (`<div`), attributes can be added
/// - **InContent**: The opening tag is closed (`>`), content can be written
///
/// # Example
///
/// ```rust
/// use plait::{Html, HtmlFormatter};
///
/// let mut output = Html::new();
/// let mut f = HtmlFormatter::new(&mut output);
///
/// f.start_element("div");
/// f.write_attribute("class", "container", None).unwrap();
/// f.write_content("Hello, world!", None).unwrap();
/// f.end_element().unwrap();
///
/// assert_eq!(&*output, "<div class=\"container\">Hello, world!</div>");
/// ```
pub struct HtmlFormatter<'a> {
    output: &'a mut Html,
    element_stack: Vec<ElementEntry>,
    state: FormatterState,
}

impl<'a> HtmlFormatter<'a> {
    /// Creates a new `HtmlFormatter` with the given output.
    pub fn new(output: &'a mut Html) -> Self {
        HtmlFormatter {
            output,
            element_stack: Vec::new(),
            state: FormatterState::Idle,
        }
    }

    /// Closes the current opening tag if one is pending.
    fn close_pending_tag(&mut self) {
        // This writes the `>` character to transition from `TagOpened` to `InContent`.
        if self.state == FormatterState::TagOpened && !self.element_stack.is_empty() {
            self.element_stack
                .last()
                .unwrap()
                .attributes
                .write_to(self.output);

            self.output.0.push('>');
            self.state = FormatterState::InContent;
        }
    }

    /// Writes a string directly to the output with the specified escape mode.
    ///
    /// This is a low-level method primarily used by [`Render`] implementations
    /// for primitive types. It writes directly to the underlying output without
    /// affecting the formatter's state machine.
    ///
    /// [`Render`]: crate::Render
    pub(crate) fn write_str(&mut self, s: &str, escape_mode: EscapeMode) {
        match escape_mode {
            EscapeMode::Raw => self.output.0.push_str(s),
            EscapeMode::Html => escape_html(self.output, s),
            EscapeMode::Url => escape_url(self.output, s),
        }
    }

    /// Returns a mutable reference to the underlying [`Html`] output.
    ///
    /// This provides direct access to the output buffer for advanced use cases.
    /// Use with caution as writing directly bypasses the formatter's state machine.
    pub(crate) fn output(&mut self) -> &mut Html {
        self.output
    }

    /// Starts a new HTML element.
    ///
    /// Note: `name` should be in ASCII lowercase for correct void element detection.
    pub fn start_element(&mut self, name: &'static str) {
        // This writes `<name` to the output and transitions to the `TagOpened` state. If currently in `TagOpened`
        // state, the pending tag is closed first.

        // Close any pending tag first
        self.close_pending_tag();

        // Write the opening tag start
        self.output.0.push('<');
        self.output.0.push_str(name);

        // Push to element stack
        self.element_stack.push(ElementEntry {
            name,
            is_void: is_void_element(name),
            attributes: Attributes::new(),
        });

        self.state = FormatterState::TagOpened;
    }

    /// Writes an attribute to the current element.
    pub fn write_attribute(
        &mut self,
        name: &'static str,
        value: impl Render,
        escape_mode: Option<EscapeMode>,
    ) -> Result<(), Error> {
        // This can only be called when in the `TagOpened` state (after `start_element` but before any content or
        // `end_element`). Returns `Error::AttributeOutsideTag` if not in `TagOpened` state.

        if self.state != FormatterState::TagOpened || self.element_stack.is_empty() {
            return Err(Error::AttributeOutsideTag);
        }

        let element = self.element_stack.last_mut().unwrap();
        element.attributes.add(name, value, escape_mode);

        Ok(())
    }

    /// Writes an optional attribute to the current element.
    pub fn write_optional_attribute(
        &mut self,
        name: &'static str,
        value: Option<impl Render>,
        escape_mode: Option<EscapeMode>,
    ) -> Result<(), Error> {
        // Optional attributes are only written when the value is `Some(_)`. Returns `Error::AttributeOutsideTag` if
        // not in `TagOpened` state.

        if self.state != FormatterState::TagOpened || self.element_stack.is_empty() {
            return Err(Error::AttributeOutsideTag);
        }

        let element = self.element_stack.last_mut().unwrap();
        element.attributes.add_optional(name, value, escape_mode);

        Ok(())
    }

    /// Writes a boolean attribute to the current element.
    pub fn write_boolean_attribute(
        &mut self,
        name: &'static str,
        value: bool,
    ) -> Result<(), Error> {
        // Boolean attributes have no value (e.g., `disabled`, `checked`). Returns `Error::AttributeOutsideTag` if not
        // in `TagOpened` state.

        if self.state != FormatterState::TagOpened || self.element_stack.is_empty() {
            return Err(Error::AttributeOutsideTag);
        }

        let element = self.element_stack.last_mut().unwrap();
        element.attributes.add_boolean(name, value);

        Ok(())
    }

    /// Spreads attributes to the current element.
    pub fn spread_attributes(&mut self, attributes: impl Into<Attributes>) -> Result<(), Error> {
        // Spread attributes are a way to apply multiple attributes to an element at once.

        if self.state != FormatterState::TagOpened || self.element_stack.is_empty() {
            return Err(Error::AttributeOutsideTag);
        }

        let element = self.element_stack.last_mut().unwrap();
        element.attributes.merge(attributes.into());

        Ok(())
    }

    /// Writes content inside the current element.
    pub fn write_content(
        &mut self,
        content: impl Render,
        escape_mode: Option<EscapeMode>,
    ) -> Result<(), Error> {
        // If in `TagOpened` state, the tag is closed first by writing `>`. The content is rendered according to the
        // `escape_mode` and current element context. Returns `Error::ContentInVoidElement` if the current element is a
        // void element.

        // Check if we're trying to write content to a void element
        if let Some(entry) = self.element_stack.last()
            && entry.is_void
            && self.state == FormatterState::TagOpened
        {
            return Err(Error::ContentInVoidElement);
        }

        // Close pending tag if needed
        self.close_pending_tag();

        // Determine escape mode based on parent element
        let element_name = self.element_stack.last().map(|e| e.name);
        let resolved_escape_mode = resolve_escape_mode_for_element(element_name, escape_mode);

        content.render_to(self, resolved_escape_mode);

        Ok(())
    }

    /// Ends the current element.
    pub fn end_element(&mut self) -> Result<(), Error> {
        // For void elements in `TagOpened` state, writes `>`. For normal elements, writes `</name>`. Returns
        // `Error::NoElementToClose` if no element is currently open.

        let entry = self.element_stack.pop().ok_or(Error::NoElementToClose)?;

        if self.state == FormatterState::TagOpened {
            entry.attributes.write_to(self.output);

            self.output.0.push('>');
        }

        // Note: void elements shouldn't have closing tags in HTML5
        if !entry.is_void {
            // Normal elements: close pending tag and write closing tag
            self.close_pending_tag();
            self.output.0.push_str("</");
            self.output.0.push_str(entry.name);
            self.output.0.push('>');
        }

        // Update state based on whether there are more elements on the stack
        self.state = if self.element_stack.is_empty() {
            FormatterState::Idle
        } else {
            FormatterState::InContent
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_string() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.write_content("Hello", None).unwrap();

        assert_eq!(output, "Hello");
    }

    #[test]
    fn test_simple_element() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.write_content("Hello", None).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div>Hello</div>");
    }

    #[test]
    fn test_element_with_attributes() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.write_attribute("class", "container", None).unwrap();
        f.write_attribute("id", "main", None).unwrap();
        f.write_content("Content", None).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div class=\"container\" id=\"main\">Content</div>");
    }

    #[test]
    fn test_nested_elements() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.start_element("span");
        f.write_content("Nested", None).unwrap();
        f.end_element().unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div><span>Nested</span></div>");
    }

    #[test]
    fn test_void_element() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.start_element("br");
        f.end_element().unwrap();
        f.start_element("input");
        f.write_attribute("type", "text", None).unwrap();
        f.end_element().unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div><br><input type=\"text\"></div>");
    }

    #[test]
    fn test_content_escaping() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.write_content("<script>alert('xss')</script>", None)
            .unwrap();
        f.end_element().unwrap();

        assert_eq!(
            output,
            "<div>&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;</div>"
        );
    }

    #[test]
    fn test_attribute_escaping() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.write_attribute("data-value", "a\"b<c>d", None).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div data-value=\"a&quot;b&lt;c&gt;d\"></div>");
    }

    #[test]
    fn test_raw_content() {
        use crate::PreEscaped;

        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.write_content(PreEscaped("<b>Bold</b>"), None).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div><b>Bold</b></div>");
    }

    #[test]
    fn test_boolean_attribute_true() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("input");
        f.write_attribute("type", "checkbox", None).unwrap();
        f.write_boolean_attribute("checked", true).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<input type=\"checkbox\" checked>");
    }

    #[test]
    fn test_boolean_attribute_false() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("input");
        f.write_attribute("type", "checkbox", None).unwrap();
        f.write_boolean_attribute("checked", false).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<input type=\"checkbox\">");
    }

    #[test]
    fn test_attribute_outside_tag_error() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.write_content("Content", None).unwrap();

        // Now in InContent state, attribute should fail
        let result = f.write_attribute("class", "test", None);
        assert!(matches!(result, Err(Error::AttributeOutsideTag)));
    }

    #[test]
    fn test_no_element_to_close_error() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        let result = f.end_element();
        assert!(matches!(result, Err(Error::NoElementToClose)));
    }

    #[test]
    fn test_content_in_void_element_error() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("br");
        let result = f.write_content("Content", None);
        assert!(matches!(result, Err(Error::ContentInVoidElement)));
    }

    #[test]
    fn test_optional_attribute_with_some() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.write_optional_attribute("class", Some("container"), None)
            .unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div class=\"container\"></div>");
    }

    #[test]
    fn test_optional_attribute_with_none() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.write_optional_attribute("class", None::<&str>, None)
            .unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div></div>");
    }

    #[test]
    fn test_optional_attribute_mixed() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.write_optional_attribute("id", Some("main"), None)
            .unwrap();
        f.write_optional_attribute("class", None::<&str>, None)
            .unwrap();
        f.write_optional_attribute("data-value", Some("test"), None)
            .unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div id=\"main\" data-value=\"test\"></div>");
    }

    #[test]
    fn test_optional_attribute_outside_tag_error() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.write_content("Content", None).unwrap();

        // Now in InContent state, optional attribute should fail
        let result = f.write_optional_attribute("class", Some("test"), None);
        assert!(matches!(result, Err(Error::AttributeOutsideTag)));
    }

    #[test]
    fn test_optional_attribute_escaping() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        f.start_element("div");
        f.write_optional_attribute("data-value", Some("a\"b<c>d"), None)
            .unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div data-value=\"a&quot;b&lt;c&gt;d\"></div>");
    }

    #[test]
    fn test_spread_attributes_basic() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        let mut attrs = Attributes::new();
        attrs.add("id", "main", None);
        attrs.add("class", "container", None);

        f.start_element("div");
        f.spread_attributes(attrs).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div id=\"main\" class=\"container\"></div>");
    }

    #[test]
    fn test_spread_attributes_empty() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        let attrs = Attributes::new();

        f.start_element("div");
        f.spread_attributes(attrs).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div></div>");
    }

    #[test]
    fn test_spread_attributes_with_existing_attributes() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        let mut attrs = Attributes::new();
        attrs.add("data-value", "spread", None);

        f.start_element("div");
        f.write_attribute("id", "main", None).unwrap();
        f.spread_attributes(attrs).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div id=\"main\" data-value=\"spread\"></div>");
    }

    #[test]
    fn test_spread_attributes_class_merging() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        let mut attrs = Attributes::new();
        attrs.add("class", "spread-class", None);

        f.start_element("div");
        f.write_attribute("class", "existing-class", None).unwrap();
        f.spread_attributes(attrs).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div class=\"existing-class spread-class\"></div>");
    }

    #[test]
    fn test_spread_attributes_with_boolean() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        let mut attrs = Attributes::new();
        attrs.add("type", "checkbox", None);
        attrs.add_boolean("checked", true);

        f.start_element("input");
        f.spread_attributes(attrs).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<input type=\"checkbox\" checked>");
    }

    #[test]
    fn test_spread_attributes_overwrites_non_class() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        let mut attrs = Attributes::new();
        attrs.add("id", "spread-id", None);

        f.start_element("div");
        f.write_attribute("id", "original-id", None).unwrap();
        f.spread_attributes(attrs).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div id=\"spread-id\"></div>");
    }

    #[test]
    fn test_spread_attributes_outside_tag_error_in_content() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        let attrs = Attributes::new();

        f.start_element("div");
        f.write_content("Content", None).unwrap();

        // Now in InContent state, spread should fail
        let result = f.spread_attributes(attrs);
        assert!(matches!(result, Err(Error::AttributeOutsideTag)));
    }

    #[test]
    fn test_spread_attributes_outside_tag_error_idle() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        let attrs = Attributes::new();

        // In Idle state (no element started), spread should fail
        let result = f.spread_attributes(attrs);
        assert!(matches!(result, Err(Error::AttributeOutsideTag)));
    }

    #[test]
    fn test_spread_attributes_multiple_spreads() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        let mut attrs1 = Attributes::new();
        attrs1.add("id", "main", None);

        let mut attrs2 = Attributes::new();
        attrs2.add("data-value", "test", None);

        f.start_element("div");
        f.spread_attributes(attrs1).unwrap();
        f.spread_attributes(attrs2).unwrap();
        f.end_element().unwrap();

        assert_eq!(output, "<div id=\"main\" data-value=\"test\"></div>");
    }

    #[test]
    fn test_spread_attributes_with_escaped_values() {
        let mut output = Html::new();
        let mut f = HtmlFormatter::new(&mut output);

        let mut attrs = Attributes::new();
        attrs.add("data-value", "<script>alert('xss')</script>", None);

        f.start_element("div");
        f.spread_attributes(attrs).unwrap();
        f.end_element().unwrap();

        assert_eq!(
            output,
            "<div data-value=\"&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;\"></div>"
        );
    }
}
