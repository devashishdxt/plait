use crate::{
    Error, EscapeMode, Render,
    escape::{resolve_escape_mode_for_attribute, resolve_escape_mode_for_element},
};

/// List of HTML void elements that cannot have content or closing tags.
const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

/// Returns true if the given element name is a void element.
fn is_void_element(name: &str) -> bool {
    VOID_ELEMENTS.contains(&name.to_ascii_lowercase().as_str())
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
struct ElementEntry<'a> {
    name: &'a str,
    is_void: bool,
}

/// Formatter for HTML output.
pub struct HtmlFormatter<'a> {
    output: &'a mut String,
    element_stack: Vec<ElementEntry<'a>>,
    state: FormatterState,
}

impl<'a> HtmlFormatter<'a> {
    /// Creates a new `HtmlFormatter` with the given output.
    pub fn new(output: &'a mut String) -> Self {
        HtmlFormatter {
            output,
            element_stack: Vec::new(),
            state: FormatterState::Idle,
        }
    }

    /// Closes the current opening tag if one is pending.
    fn close_pending_tag(&mut self) {
        // This writes the `>` character to transition from `TagOpened` to `InContent`.

        if self.state == FormatterState::TagOpened {
            self.output.push('>');
            self.state = FormatterState::InContent;
        }
    }

    /// Starts a new HTML element.
    pub fn start_element(&mut self, name: &'a str) {
        // This writes `<name` to the output and transitions to the `TagOpened` state. If currently in `TagOpened`
        // state, the pending tag is closed first.

        // Close any pending tag first
        self.close_pending_tag();

        // Write the opening tag start
        self.output.push('<');
        self.output.push_str(name);

        // Push to element stack
        self.element_stack.push(ElementEntry {
            name,
            is_void: is_void_element(name),
        });

        self.state = FormatterState::TagOpened;
    }

    /// Writes an attribute to the current element.
    pub fn write_attribute(
        &mut self,
        name: &str,
        value: impl Render,
        escape_mode: Option<EscapeMode>,
    ) -> Result<(), Error> {
        // This can only be called when in the `TagOpened` state (after `start_element` but before any content or
        // `end_element`). Returns `Error::AttributeOutsideTag` if not in `TagOpened` state.

        if self.state != FormatterState::TagOpened || self.element_stack.is_empty() {
            return Err(Error::AttributeOutsideTag);
        }

        let resolved_escape_mode = resolve_escape_mode_for_attribute(
            self.element_stack.last().unwrap().name,
            name,
            escape_mode,
        );

        self.output.push(' ');
        self.output.push_str(name);
        self.output.push_str("=\"");
        value.render(&mut self.output, resolved_escape_mode)?;
        self.output.push('"');
        Ok(())
    }

    /// Writes an optional attribute to the current element.
    pub fn write_optional_attribute(
        &mut self,
        name: &str,
        value: Option<impl Render>,
        escape_mode: Option<EscapeMode>,
    ) -> Result<(), Error> {
        // Optional attributes are only written when the value is `Some(_)`. Returns `Error::AttributeOutsideTag` if
        // not in `TagOpened` state.

        if self.state != FormatterState::TagOpened || self.element_stack.is_empty() {
            return Err(Error::AttributeOutsideTag);
        }

        if let Some(value) = value {
            let resolved_escape_mode = resolve_escape_mode_for_attribute(
                self.element_stack.last().unwrap().name,
                name,
                escape_mode,
            );

            self.output.push(' ');
            self.output.push_str(name);
            self.output.push_str("=\"");
            value.render(&mut self.output, resolved_escape_mode)?;
            self.output.push('"');
        }

        Ok(())
    }

    /// Writes a boolean attribute to the current element.
    pub fn write_boolean_attribute(&mut self, name: &str, value: bool) -> Result<(), Error> {
        // Boolean attributes have no value (e.g., `disabled`, `checked`). Returns `Error::AttributeOutsideTag` if not
        // in `TagOpened` state.

        if self.state != FormatterState::TagOpened || self.element_stack.is_empty() {
            return Err(Error::AttributeOutsideTag);
        }

        if value {
            self.output.push(' ');
            self.output.push_str(name);
        }

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
        if let Some(entry) = self.element_stack.last() {
            if entry.is_void && self.state == FormatterState::TagOpened {
                return Err(Error::ContentInVoidElement);
            }
        }

        // Close pending tag if needed
        self.close_pending_tag();

        // Determine escape mode based on parent element
        let element_name = self.element_stack.last().map(|e| e.name);
        let resolved_escape_mode = resolve_escape_mode_for_element(element_name, escape_mode);

        content.render(&mut self.output, resolved_escape_mode)?;

        Ok(())
    }

    /// Ends the current element.
    pub fn end_element(&mut self) -> Result<(), Error> {
        // For void elements in `TagOpened` state, writes `>`. For normal elements, writes `</name>`. Returns
        // `Error::NoElementToClose` if no element is currently open.

        let entry = self.element_stack.pop().ok_or(Error::NoElementToClose)?;

        if entry.is_void {
            // Void elements: just close the tag with >
            if self.state == FormatterState::TagOpened {
                self.output.push('>');
            }
            // Note: void elements shouldn't have closing tags in HTML5
        } else {
            // Normal elements: close pending tag and write closing tag
            self.close_pending_tag();
            self.output.push_str("</");
            self.output.push_str(entry.name);
            self.output.push('>');
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
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.write_content("Hello", None).unwrap();

        assert_eq!(output, "Hello");
    }

    #[test]
    fn test_simple_element() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.write_content("Hello", None).unwrap();
        fmt.end_element().unwrap();

        assert_eq!(output, "<div>Hello</div>");
    }

    #[test]
    fn test_element_with_attributes() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.write_attribute("class", "container", None).unwrap();
        fmt.write_attribute("id", "main", None).unwrap();
        fmt.write_content("Content", None).unwrap();
        fmt.end_element().unwrap();

        assert_eq!(output, "<div class=\"container\" id=\"main\">Content</div>");
    }

    #[test]
    fn test_nested_elements() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.start_element("span");
        fmt.write_content("Nested", None).unwrap();
        fmt.end_element().unwrap();
        fmt.end_element().unwrap();

        assert_eq!(output, "<div><span>Nested</span></div>");
    }

    #[test]
    fn test_void_element() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.start_element("br");
        fmt.end_element().unwrap();
        fmt.start_element("input");
        fmt.write_attribute("type", "text", None).unwrap();
        fmt.end_element().unwrap();
        fmt.end_element().unwrap();

        assert_eq!(output, "<div><br><input type=\"text\"></div>");
    }

    #[test]
    fn test_content_escaping() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.write_content("<script>alert('xss')</script>", None)
            .unwrap();
        fmt.end_element().unwrap();

        assert_eq!(
            output,
            "<div>&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;</div>"
        );
    }

    #[test]
    fn test_attribute_escaping() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.write_attribute("data-value", "a\"b<c>d", None).unwrap();
        fmt.end_element().unwrap();

        assert_eq!(output, "<div data-value=\"a&quot;b&lt;c&gt;d\"></div>");
    }

    #[test]
    fn test_raw_content() {
        use crate::PreEscaped;

        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.write_content(PreEscaped("<b>Bold</b>"), None).unwrap();
        fmt.end_element().unwrap();

        assert_eq!(output, "<div><b>Bold</b></div>");
    }

    #[test]
    fn test_boolean_attribute_true() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("input");
        fmt.write_attribute("type", "checkbox", None).unwrap();
        fmt.write_boolean_attribute("checked", true).unwrap();
        fmt.end_element().unwrap();

        assert_eq!(output, "<input type=\"checkbox\" checked>");
    }

    #[test]
    fn test_boolean_attribute_false() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("input");
        fmt.write_attribute("type", "checkbox", None).unwrap();
        fmt.write_boolean_attribute("checked", false).unwrap();
        fmt.end_element().unwrap();

        assert_eq!(output, "<input type=\"checkbox\">");
    }

    #[test]
    fn test_attribute_outside_tag_error() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.write_content("Content", None).unwrap();

        // Now in InContent state, attribute should fail
        let result = fmt.write_attribute("class", "test", None);
        assert!(matches!(result, Err(Error::AttributeOutsideTag)));
    }

    #[test]
    fn test_no_element_to_close_error() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        let result = fmt.end_element();
        assert!(matches!(result, Err(Error::NoElementToClose)));
    }

    #[test]
    fn test_content_in_void_element_error() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("br");
        let result = fmt.write_content("Content", None);
        assert!(matches!(result, Err(Error::ContentInVoidElement)));
    }

    #[test]
    fn test_optional_attribute_with_some() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.write_optional_attribute("class", Some("container"), None)
            .unwrap();
        fmt.end_element().unwrap();

        assert_eq!(output, "<div class=\"container\"></div>");
    }

    #[test]
    fn test_optional_attribute_with_none() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.write_optional_attribute("class", None::<&str>, None)
            .unwrap();
        fmt.end_element().unwrap();

        assert_eq!(output, "<div></div>");
    }

    #[test]
    fn test_optional_attribute_mixed() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.write_optional_attribute("id", Some("main"), None)
            .unwrap();
        fmt.write_optional_attribute("class", None::<&str>, None)
            .unwrap();
        fmt.write_optional_attribute("data-value", Some("test"), None)
            .unwrap();
        fmt.end_element().unwrap();

        assert_eq!(output, "<div id=\"main\" data-value=\"test\"></div>");
    }

    #[test]
    fn test_optional_attribute_outside_tag_error() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.write_content("Content", None).unwrap();

        // Now in InContent state, optional attribute should fail
        let result = fmt.write_optional_attribute("class", Some("test"), None);
        assert!(matches!(result, Err(Error::AttributeOutsideTag)));
    }

    #[test]
    fn test_optional_attribute_escaping() {
        let mut output = String::new();
        let mut fmt = HtmlFormatter::new(&mut output);

        fmt.start_element("div");
        fmt.write_optional_attribute("data-value", Some("a\"b<c>d"), None)
            .unwrap();
        fmt.end_element().unwrap();

        assert_eq!(output, "<div data-value=\"a&quot;b&lt;c&gt;d\"></div>");
    }
}
