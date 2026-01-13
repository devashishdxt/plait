use indexmap::IndexMap;

use crate::{Error, EscapeMode, Html, Render, escape::resolve_escape_mode_for_attribute};

/// Collection of attributes for an HTML element.
pub struct Attributes {
    attributes: IndexMap<&'static str, Option<Html>>,
}

impl Attributes {
    /// Creates a new instance of `Attributes`.
    pub fn new() -> Self {
        Attributes {
            attributes: IndexMap::new(),
        }
    }

    /// Adds an attribute to the collection.
    pub fn add(
        &mut self,
        parent: &'static str,
        name: &'static str,
        value: impl Render,
        escape_mode: Option<EscapeMode>,
    ) -> Result<(), Error> {
        let resolved_escape_mode = resolve_escape_mode_for_attribute(parent, name, escape_mode);

        if name == "class" {
            let existing = self.attributes.get_mut(name);

            if let Some(Some(existing)) = existing {
                existing.0.push(' ');
                value.render_to(existing, resolved_escape_mode)?;
            } else {
                self.attributes
                    .insert(name, Some(value.render(resolved_escape_mode)?));
            }
        } else {
            self.attributes
                .insert(name, Some(value.render(resolved_escape_mode)?));
        }

        Ok(())
    }

    /// Adds an optional attribute to the collection.
    pub fn add_optional(
        &mut self,
        parent: &'static str,
        name: &'static str,
        value: Option<impl Render>,
        escape_mode: Option<EscapeMode>,
    ) -> Result<(), Error> {
        if let Some(value) = value {
            self.add(parent, name, value, escape_mode)?;
        }

        Ok(())
    }

    /// Adds a boolean attribute to the collection.
    pub fn add_boolean(&mut self, name: &'static str, value: bool) -> Result<(), Error> {
        if value {
            self.attributes.insert(name, None);
        }

        Ok(())
    }

    /// Merges another set of attributes into this collection.
    pub fn merge(&mut self, other: Attributes) {
        for (name, value) in other.attributes {
            if name == "class" {
                if let Some(value) = value {
                    let existing = self.attributes.get_mut(name);

                    if let Some(Some(existing)) = existing {
                        existing.0.push(' ');
                        value.render_to(existing, EscapeMode::Raw).unwrap();
                    } else {
                        self.attributes.insert(name, Some(value));
                    }
                }
            } else {
                self.attributes.insert(name, value);
            }
        }
    }
}

impl Render for Attributes {
    fn render_to(&self, output: &mut Html, _escape_mode: EscapeMode) -> Result<(), Error> {
        println!("Rendering");
        for (name, value) in self.attributes.iter() {
            output.0.push(' ');
            output.0.push_str(name);

            if let Some(value) = value {
                output.0.push_str("=\"");
                output.0.push_str(&value.0);
                output.0.push('"');
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_new_creates_empty_attributes() {
        let attrs = Attributes::new();
        assert!(attrs.attributes.is_empty());
        assert_eq!(attrs.attributes.len(), 0);
    }

    #[test]
    fn test_add_single_attribute() {
        let mut attrs = Attributes::new();
        attrs.add("div", "id", "main", None).unwrap();

        assert_eq!(attrs.attributes.len(), 1);
        assert_eq!(get_value(&attrs, "id"), "main");
    }

    #[test]
    fn test_add_multiple_different_attributes() {
        let mut attrs = Attributes::new();
        attrs.add("div", "id", "main", None).unwrap();
        attrs.add("div", "data-value", "test", None).unwrap();

        assert_eq!(attrs.attributes.len(), 2);
        assert_eq!(get_value(&attrs, "id"), "main");
        assert_eq!(get_value(&attrs, "data-value"), "test");
    }

    #[test]
    fn test_add_non_class_attribute_overwrites() {
        let mut attrs = Attributes::new();
        attrs.add("div", "id", "first", None).unwrap();
        attrs.add("div", "id", "second", None).unwrap();

        assert_eq!(attrs.attributes.len(), 1);
        assert_eq!(get_value(&attrs, "id"), "second");
    }

    #[test]
    fn test_add_class_attribute_merges() {
        let mut attrs = Attributes::new();
        attrs.add("div", "class", "foo", None).unwrap();
        attrs.add("div", "class", "bar", None).unwrap();

        assert_eq!(attrs.attributes.len(), 1);
        assert_eq!(get_value(&attrs, "class"), "foo bar");
    }

    #[test]
    fn test_add_class_attribute_merges_multiple() {
        let mut attrs = Attributes::new();
        attrs.add("div", "class", "a", None).unwrap();
        attrs.add("div", "class", "b", None).unwrap();
        attrs.add("div", "class", "c", None).unwrap();

        assert_eq!(attrs.attributes.len(), 1);
        assert_eq!(get_value(&attrs, "class"), "a b c");
    }

    #[test]
    fn test_add_escapes_html_in_attribute_value() {
        let mut attrs = Attributes::new();
        attrs.add("div", "data-value", "<script>", None).unwrap();

        assert_eq!(get_value(&attrs, "data-value"), "&lt;script&gt;");
    }

    #[test]
    fn test_add_escapes_quotes_in_attribute_value() {
        let mut attrs = Attributes::new();
        attrs.add("div", "data-value", "a\"b", None).unwrap();

        assert_eq!(get_value(&attrs, "data-value"), "a&quot;b");
    }

    #[test]
    fn test_add_optional_with_some_value() {
        let mut attrs = Attributes::new();
        attrs.add_optional("div", "id", Some("main"), None).unwrap();

        assert_eq!(attrs.attributes.len(), 1);
        assert_eq!(get_value(&attrs, "id"), "main");
    }

    #[test]
    fn test_add_optional_with_none_value() {
        let mut attrs = Attributes::new();
        attrs.add_optional("div", "id", None::<&str>, None).unwrap();

        assert!(attrs.attributes.is_empty());
    }

    #[test]
    fn test_add_optional_class_merges() {
        let mut attrs = Attributes::new();
        attrs
            .add_optional("div", "class", Some("foo"), None)
            .unwrap();
        attrs
            .add_optional("div", "class", Some("bar"), None)
            .unwrap();

        assert_eq!(attrs.attributes.len(), 1);
        assert_eq!(get_value(&attrs, "class"), "foo bar");
    }

    #[test]
    fn test_add_boolean_true() {
        let mut attrs = Attributes::new();
        attrs.add_boolean("disabled", true).unwrap();

        assert_eq!(attrs.attributes.len(), 1);
        assert!(attrs.attributes.get("disabled").unwrap().is_none());
    }

    #[test]
    fn test_add_boolean_false() {
        let mut attrs = Attributes::new();
        attrs.add_boolean("disabled", false).unwrap();

        assert!(attrs.attributes.is_empty());
    }

    #[test]
    fn test_add_boolean_multiple() {
        let mut attrs = Attributes::new();
        attrs.add_boolean("disabled", true).unwrap();
        attrs.add_boolean("checked", true).unwrap();
        attrs.add_boolean("readonly", false).unwrap();

        assert_eq!(attrs.attributes.len(), 2);
        assert!(attrs.attributes.get("disabled").is_some());
        assert!(attrs.attributes.get("checked").is_some());
        assert!(attrs.attributes.get("readonly").is_none());
    }

    #[test]
    fn test_mixed_attributes() {
        let mut attrs = Attributes::new();
        attrs.add("input", "type", "checkbox", None).unwrap();
        attrs.add("input", "class", "form-check", None).unwrap();
        attrs.add("input", "class", "mt-2", None).unwrap();
        attrs.add_boolean("checked", true).unwrap();
        attrs
            .add_optional("input", "id", Some("my-checkbox"), None)
            .unwrap();
        attrs
            .add_optional("input", "name", None::<&str>, None)
            .unwrap();

        assert_eq!(attrs.attributes.len(), 4); // type, class, checked, id
        assert_eq!(get_value(&attrs, "type"), "checkbox");
        assert_eq!(get_value(&attrs, "class"), "form-check mt-2");
        assert!(attrs.attributes.get("checked").unwrap().is_none()); // boolean attr
        assert_eq!(get_value(&attrs, "id"), "my-checkbox");
        assert!(attrs.attributes.get("name").is_none()); // was None, not added
    }

    #[test]
    fn test_numeric_attribute_values() {
        let mut attrs = Attributes::new();
        attrs.add("div", "data-count", 42i32, None).unwrap();
        attrs.add("div", "data-price", 19.99f64, None).unwrap();

        assert_eq!(get_value(&attrs, "data-count"), "42");
        assert_eq!(get_value(&attrs, "data-price"), "19.99");
    }

    // Tests for Render implementation

    /// Helper to render attributes to string
    fn render_attrs(attrs: &Attributes) -> String {
        attrs.render(EscapeMode::Html).unwrap().0
    }

    #[test]
    fn test_render_empty_attributes() {
        let attrs = Attributes::new();
        assert_eq!(render_attrs(&attrs), "");
    }

    #[test]
    fn test_render_single_valued_attribute() {
        let mut attrs = Attributes::new();
        attrs.add("div", "id", "main", None).unwrap();

        assert_eq!(render_attrs(&attrs), " id=\"main\"");
    }

    #[test]
    fn test_render_single_boolean_attribute() {
        let mut attrs = Attributes::new();
        attrs.add_boolean("disabled", true).unwrap();

        assert_eq!(render_attrs(&attrs), " disabled");
    }

    #[test]
    fn test_render_preserves_escaped_values() {
        let mut attrs = Attributes::new();
        attrs.add("div", "data-value", "<script>", None).unwrap();

        // Values are escaped when added, render should not double-escape
        assert_eq!(render_attrs(&attrs), " data-value=\"&lt;script&gt;\"");
    }

    #[test]
    fn test_render_multiple_attributes_contains_all() {
        let mut attrs = Attributes::new();
        attrs.add("div", "id", "main", None).unwrap();
        attrs.add("div", "class", "container", None).unwrap();
        attrs.add_boolean("hidden", true).unwrap();

        let rendered = render_attrs(&attrs);

        // HashMap order is not guaranteed, so check contains
        assert!(rendered.contains(" id=\"main\""));
        assert!(rendered.contains(" class=\"container\""));
        assert!(rendered.contains(" hidden"));
    }

    #[test]
    fn test_render_merged_class_attribute() {
        let mut attrs = Attributes::new();
        attrs.add("div", "class", "foo", None).unwrap();
        attrs.add("div", "class", "bar", None).unwrap();

        assert_eq!(render_attrs(&attrs), " class=\"foo bar\"");
    }
}
