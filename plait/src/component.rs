use crate::HtmlFormatter;

/// A trait for creating reusable HTML components.
///
/// Components encapsulate HTML structure and behavior, allowing you to define reusable UI elements with their own
/// properties while still accepting additional attributes and children from the call site.
///
/// # Usage
///
/// Use the [`component!`](crate::component!) macro to define components:
///
/// ```rust
/// use plait::{component, html, merge_classes, render};
///
/// component! {
///     pub fn Button<'a>(class: Option<&'a str>, disabled: bool) {
///         button(class: merge_classes!("btn", class), disabled?: disabled, #attrs) {
///             #children
///         }
///     }
/// }
///
/// let html = render(html! {
///     @Button(class: Some("btn-primary"), disabled: false; id: "button-id") {
///         "Click me"
///     }
/// });
///
/// assert_eq!(html, r#"<button class="btn btn-primary" id="button-id">Click me</button>"#);
/// ```
///
/// The `component!` macro generates a struct and implements this trait automatically. See
/// [`component!`](crate::component!) for the full syntax reference.
pub trait Component {
    /// Renders the component to the given HTML formatter.
    fn render(
        self,
        f: &mut HtmlFormatter<'_>,
        attrs: impl FnOnce(&mut HtmlFormatter<'_>),
        children: impl FnOnce(&mut HtmlFormatter<'_>),
    );
}

#[cfg(test)]
mod tests {
    use crate::{HtmlFragment, merge_classes, render};

    use super::*;

    struct Button<'a> {
        class: &'a str,
        disabled: bool,
    }

    impl<'a> Component for Button<'a> {
        fn render(
            self,
            f: &mut HtmlFormatter<'_>,
            attrs: impl FnOnce(&mut HtmlFormatter<'_>),
            children: impl FnOnce(&mut HtmlFormatter<'_>),
        ) {
            f.open_tag("button");
            f.write_attribute_escaped("class", merge_classes!("btn", self.class));
            f.write_boolean_attribute("disabled", self.disabled);
            attrs(f);
            f.close_start_tag();
            children(f);
            f.close_tag("button");
        }
    }

    #[test]
    fn test_button_component() {
        let html = render(HtmlFragment(|f| {
            Button {
                class: "btn-primary",
                disabled: false,
            }
            .render(
                f,
                |f| {
                    f.write_attribute_escaped("id", "button-id");
                },
                |f| {
                    f.write_html_escaped("Click me");
                },
            );
        }));

        assert_eq!(
            html,
            r#"<button class="btn btn-primary" id="button-id">Click me</button>"#
        );
    }

    #[test]
    fn test_button_component_disabled() {
        let html = render(HtmlFragment(|f| {
            Button {
                class: "btn-primary",
                disabled: true,
            }
            .render(
                f,
                |f| {
                    f.write_attribute_escaped("id", "button-id");
                },
                |f| {
                    f.write_html_escaped("Click me");
                },
            );
        }));

        assert_eq!(
            html,
            r#"<button class="btn btn-primary" disabled id="button-id">Click me</button>"#
        );
    }
}
