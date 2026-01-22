use plait::{Attributes, Render, component};

pub fn button<'a>(label: &'a str, primary: bool, attrs: &'a Attributes) -> impl Render + 'a {
    let class = if primary { "btn btn-primary" } else { "btn" };
    component! {
        button class=(class) ..(attrs) { (label) }
    }
}

#[cfg(test)]
mod tests {
    use plait::attrs;

    use super::*;

    #[test]
    fn test_button_rendering() {
        let attrs = attrs!(class = "dark");
        let btn = button("Click me", true, &attrs);
        let output = plait::html!(
            div { (btn) }
        );

        assert_eq!(
            &*output,
            r#"<div><button class="btn btn-primary dark">Click me</button></div>"#
        );
    }
}
