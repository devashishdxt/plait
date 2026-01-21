use plait::{Attributes, EscapeMode, HtmlFormatter, Render, render};

pub struct Button {
    label: String,
    primary: bool,
    attrs: Attributes,
}

impl Render for Button {
    fn render_to(&self, f: &mut HtmlFormatter, _escape_mode: EscapeMode) {
        let class = if self.primary {
            "btn btn-primary"
        } else {
            "btn"
        };
        render!(f, {
            button class=(class) ..(&self.attrs) { (&self.label) }
        });
    }
}

#[cfg(test)]
mod tests {
    use plait::attrs;

    use super::*;

    #[test]
    fn test_button_rendering() {
        // Use in templates
        let btn = Button {
            label: "Click me".into(),
            primary: true,
            attrs: attrs!(class = "dark"),
        };
        let output = plait::html!(
            div { (btn) }
        );

        assert_eq!(
            &*output,
            r#"<div><button class="btn btn-primary dark">Click me</button></div>"#
        );
    }
}
