use indexmap::{IndexMap, IndexSet};

use crate::{EscapeMode, Html, HtmlFormatter, Render, render};

/// A collection of HTML element attributes.
///
/// `Attributes` provides a builder-style API for constructing HTML element attributes, with special handling for
/// class attributes (which are merged rather than overwritten) and boolean attributes (rendered without a value,
/// e.g., `disabled`).
///
/// Values are automatically escaped according to their attribute name context (e.g., URL validation and HTML escaping
/// for `href`, `src`, etc.).
///
/// # Using the `attrs!` Macro
///
/// The [`attrs!`](crate::attrs) macro provides a concise way to construct attributes:
///
/// ```rust
/// use plait::{attrs, render};
///
/// let class_name = "container";
/// let maybe_title: Option<&str> = Some("Hello");
/// let is_disabled = true;
///
/// let attrs = attrs!(
///     id="main"                   // Literal string value
///     class=(class_name)          // Dynamic value from expression
///     title=[maybe_title]         // Optional value (only added if Some)
///     disabled?[is_disabled]      // Boolean attribute (added if condition is true)
///     hidden                      // Boolean attribute (always added)
/// );
///
/// assert_eq!(render(attrs), r#"class="container" id="main" title="Hello" disabled hidden"#);
/// ```
///
/// ## Macro Syntax
///
/// | Syntax                 | Description                                            |
/// |------------------------|--------------------------------------------------------|
/// | `name="literal"`       | Literal string value                                   |
/// | `name=(expr)`          | Dynamic value from expression                          |
/// | `name=[optional_expr]` | Optional value (only rendered if `Some`)               |
/// | `name?[bool_expr]`     | Boolean attribute (rendered if condition is `true`)    |
/// | `name`                 | Boolean attribute (always rendered)                    |
/// | `..(spread_expr)`      | Spread attributes from another `Attributes` collection |
///
/// # Builder API Example
///
/// ```rust
/// use plait::{Attributes, render};
///
/// let mut attrs = Attributes::new();
/// attrs.add("id", "main", None);
/// attrs.add("class", "container", None);
/// attrs.add("class", "flex", None);  // Classes are merged
/// attrs.add_boolean("disabled", true);
///
/// assert_eq!(render(attrs), "class=\"container flex\" id=\"main\" disabled");
/// ```
#[derive(Default, Clone)]
pub struct Attributes {
    classes: IndexSet<String>,
    attributes: IndexMap<&'static str, Option<Html>>,
}

impl Attributes {
    /// Creates a new instance of `Attributes`.
    pub fn new() -> Self {
        Attributes {
            classes: IndexSet::new(),
            attributes: IndexMap::new(),
        }
    }

    /// Creates a new instance of `Attributes` with a specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Attributes {
            classes: IndexSet::new(),
            attributes: IndexMap::with_capacity(capacity),
        }
    }

    /// Adds an attribute to the collection.
    pub fn add(&mut self, name: &'static str, value: impl Render, escape_mode: Option<EscapeMode>) {
        let resolved_escape_mode = EscapeMode::resolve_for_attribute(name, escape_mode);

        if name == "class" {
            let rendered = render(value);
            let new_classes = rendered.split(' ');

            for new_class in new_classes {
                if !new_class.is_empty() {
                    self.classes.insert(new_class.to_owned());
                }
            }
        } else {
            self.attributes
                .insert(name, Some(render_with_mode(value, resolved_escape_mode)));
        }
    }

    /// Adds an optional attribute to the collection.
    pub fn add_optional(
        &mut self,
        name: &'static str,
        value: Option<impl Render>,
        escape_mode: Option<EscapeMode>,
    ) {
        if let Some(value) = value {
            self.add(name, value, escape_mode);
        }
    }

    /// Adds a boolean attribute to the collection.
    pub fn add_boolean(&mut self, name: &'static str, value: bool) {
        if value {
            self.attributes.insert(name, None);
        }
    }

    /// Merges another set of attributes into this collection.
    pub fn merge(&mut self, other: Attributes) {
        for class in other.classes {
            self.classes.insert(class);
        }

        for (name, value) in other.attributes {
            self.attributes.insert(name, value);
        }
    }

    /// Writes the attributes directly to an Html output.
    ///
    /// This is a low-level method used internally by [`HtmlFormatter`].
    pub(crate) fn write_to(&self, output: &mut Html) {
        if !self.classes.is_empty() {
            output.inner_mut().push_str("class=\"");

            for class in &self.classes {
                output.inner_mut().push_str(class);
                output.inner_mut().push(' ');
            }

            output.inner_mut().pop();
            output.inner_mut().push_str("\" ");
        }

        for (name, value) in self.attributes.iter() {
            output.inner_mut().push_str(name);

            if let Some(value) = value {
                output.inner_mut().push_str("=\"");
                output.inner_mut().push_str(value);
                output.inner_mut().push('"');
            }
            output.inner_mut().push(' ');
        }

        output.inner_mut().pop();
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.classes.is_empty() && self.attributes.is_empty()
    }
}

impl From<&Attributes> for Attributes {
    fn from(attributes: &Attributes) -> Self {
        attributes.clone()
    }
}

impl Render for Attributes {
    fn render_html(&self, f: &mut HtmlFormatter) {
        self.render_raw(f);
    }

    fn render_url(&self, f: &mut HtmlFormatter) {
        self.render_raw(f);
    }

    fn render_raw(&self, f: &mut HtmlFormatter) {
        self.write_to(f.output());
    }
}

/// Renders a value into an HTML string with the specified escape mode.
fn render_with_mode(value: impl Render, mode: EscapeMode) -> Html {
    let mut output = Html::new();
    let mut f = HtmlFormatter::new(&mut output);
    value.render_to(&mut f, mode);
    output
}

#[cfg(test)]
mod tests {
    use crate::render;

    use super::*;

    /// Helper to get attribute value as &str for comparison
    fn get_value<'a>(attrs: &'a Attributes, name: &'static str) -> &'a str {
        attrs
            .attributes
            .get(name)
            .unwrap()
            .as_ref()
            .unwrap()
            .as_ref()
    }

    fn has_class(attrs: &Attributes, class: &str) -> bool {
        attrs.classes.contains(class)
    }

    #[test]
    fn test_new_creates_empty_attributes() {
        let attrs = Attributes::new();
        assert!(attrs.attributes.is_empty());
        assert_eq!(attrs.attributes.len(), 0);
    }

    #[test]
    fn test_add_single_attribute() {
        let mut attrs = Attributes::new();
        attrs.add("id", "main", None);

        assert_eq!(attrs.attributes.len(), 1);
        assert_eq!(get_value(&attrs, "id"), "main");
    }

    #[test]
    fn test_add_multiple_different_attributes() {
        let mut attrs = Attributes::new();
        attrs.add("id", "main", None);
        attrs.add("data-value", "test", None);

        assert_eq!(attrs.attributes.len(), 2);
        assert_eq!(get_value(&attrs, "id"), "main");
        assert_eq!(get_value(&attrs, "data-value"), "test");
    }

    #[test]
    fn test_add_non_class_attribute_overwrites() {
        let mut attrs = Attributes::new();
        attrs.add("id", "first", None);
        attrs.add("id", "second", None);

        assert_eq!(attrs.attributes.len(), 1);
        assert_eq!(get_value(&attrs, "id"), "second");
    }

    #[test]
    fn test_add_class_attribute_merges() {
        let mut attrs = Attributes::new();
        attrs.add("class", "foo", None);
        attrs.add("class", "bar", None);

        assert_eq!(attrs.classes.len(), 2);
        assert!(has_class(&attrs, "foo"));
        assert!(has_class(&attrs, "bar"));
    }

    #[test]
    fn test_add_class_attribute_merges_multiple() {
        let mut attrs = Attributes::new();
        attrs.add("class", "a", None);
        attrs.add("class", "b", None);
        attrs.add("class", "c", None);

        assert_eq!(attrs.classes.len(), 3);
        assert!(has_class(&attrs, "a"));
        assert!(has_class(&attrs, "b"));
        assert!(has_class(&attrs, "c"));
    }

    #[test]
    fn test_add_class_attribute_merges_multiple_with_spaces() {
        let mut attrs = Attributes::new();
        attrs.add("class", "a b", None);
        attrs.add("class", "c d", None);

        assert_eq!(attrs.classes.len(), 4);
        assert!(has_class(&attrs, "a"));
        assert!(has_class(&attrs, "b"));
        assert!(has_class(&attrs, "c"));
        assert!(has_class(&attrs, "d"));
    }

    #[test]
    fn test_add_class_attribute_merges_multiple_with_many_spaces() {
        let mut attrs = Attributes::new();
        attrs.add("class", "a  b", None);
        attrs.add("class", "a  c  d", None);

        assert_eq!(attrs.classes.len(), 4);
        assert!(has_class(&attrs, "a"));
        assert!(has_class(&attrs, "b"));
        assert!(has_class(&attrs, "c"));
        assert!(has_class(&attrs, "d"));
    }

    #[test]
    fn test_add_escapes_html_in_attribute_value() {
        let mut attrs = Attributes::new();
        attrs.add("data-value", "<script>", None);

        assert_eq!(get_value(&attrs, "data-value"), "&lt;script&gt;");
    }

    #[test]
    fn test_add_escapes_quotes_in_attribute_value() {
        let mut attrs = Attributes::new();
        attrs.add("data-value", "a\"b", None);

        assert_eq!(get_value(&attrs, "data-value"), "a&quot;b");
    }

    #[test]
    fn test_add_optional_with_some_value() {
        let mut attrs = Attributes::new();
        attrs.add_optional("id", Some("main"), None);

        assert_eq!(attrs.attributes.len(), 1);
        assert_eq!(get_value(&attrs, "id"), "main");
    }

    #[test]
    fn test_add_optional_with_none_value() {
        let mut attrs = Attributes::new();
        attrs.add_optional("id", None::<&str>, None);

        assert!(attrs.attributes.is_empty());
    }

    #[test]
    fn test_add_optional_class_merges() {
        let mut attrs = Attributes::new();
        attrs.add_optional("class", Some("foo"), None);
        attrs.add_optional("class", Some("bar"), None);

        assert_eq!(attrs.classes.len(), 2);
        assert!(has_class(&attrs, "foo"));
        assert!(has_class(&attrs, "bar"));
    }

    #[test]
    fn test_add_boolean_true() {
        let mut attrs = Attributes::new();
        attrs.add_boolean("disabled", true);

        assert_eq!(attrs.attributes.len(), 1);
        assert!(attrs.attributes.get("disabled").unwrap().is_none());
    }

    #[test]
    fn test_add_boolean_false() {
        let mut attrs = Attributes::new();
        attrs.add_boolean("disabled", false);

        assert!(attrs.attributes.is_empty());
    }

    #[test]
    fn test_add_boolean_multiple() {
        let mut attrs = Attributes::new();
        attrs.add_boolean("disabled", true);
        attrs.add_boolean("checked", true);
        attrs.add_boolean("readonly", false);

        assert_eq!(attrs.attributes.len(), 2);
        assert!(attrs.attributes.get("disabled").is_some());
        assert!(attrs.attributes.get("checked").is_some());
        assert!(attrs.attributes.get("readonly").is_none());
    }

    #[test]
    fn test_mixed_attributes() {
        let mut attrs = Attributes::new();
        attrs.add("type", "checkbox", None);
        attrs.add("class", "form-check", None);
        attrs.add("class", "mt-2", None);
        attrs.add_boolean("checked", true);
        attrs.add_optional("id", Some("my-checkbox"), None);
        attrs.add_optional("name", None::<&str>, None);

        assert_eq!(attrs.attributes.len(), 3); // type, checked, id
        assert_eq!(attrs.classes.len(), 2);
        assert!(has_class(&attrs, "form-check"));
        assert!(has_class(&attrs, "mt-2"));
        assert_eq!(get_value(&attrs, "type"), "checkbox");
        assert!(attrs.attributes.get("checked").unwrap().is_none()); // boolean attr
        assert_eq!(get_value(&attrs, "id"), "my-checkbox");
        assert!(attrs.attributes.get("name").is_none()); // was None, not added
    }

    #[test]
    fn test_numeric_attribute_values() {
        let mut attrs = Attributes::new();
        attrs.add("data-count", 42i32, None);
        attrs.add("data-price", 19.99f64, None);

        assert_eq!(get_value(&attrs, "data-count"), "42");
        assert_eq!(get_value(&attrs, "data-price"), "19.99");
    }

    // Tests for Render implementation

    #[test]
    fn test_render_empty_attributes() {
        let attrs = Attributes::new();
        assert_eq!(render(&attrs), "");
    }

    #[test]
    fn test_render_single_valued_attribute() {
        let mut attrs = Attributes::new();
        attrs.add("id", "main", None);

        assert_eq!(render(&attrs), "id=\"main\"");
    }

    #[test]
    fn test_render_single_boolean_attribute() {
        let mut attrs = Attributes::new();
        attrs.add_boolean("disabled", true);

        assert_eq!(render(&attrs), "disabled");
    }

    #[test]
    fn test_render_preserves_escaped_values() {
        let mut attrs = Attributes::new();
        attrs.add("data-value", "<script>", None);

        // Values are escaped when added, render should not double-escape
        assert_eq!(render(&attrs), "data-value=\"&lt;script&gt;\"");
    }

    #[test]
    fn test_render_multiple_attributes_contains_all() {
        let mut attrs = Attributes::new();
        attrs.add("id", "main", None);
        attrs.add("class", "container", None);
        attrs.add_boolean("hidden", true);

        assert_eq!(render(&attrs), "class=\"container\" id=\"main\" hidden");
    }

    #[test]
    fn test_render_merged_class_attribute() {
        let mut attrs = Attributes::new();
        attrs.add("class", "foo", None);
        attrs.add("class", "bar", None);

        assert_eq!(render(&attrs), "class=\"foo bar\"");
    }
}
