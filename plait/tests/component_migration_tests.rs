use std::fmt;

use plait::{Component, RenderEscaped, ToHtml, component, html};

struct Handwritten<'a> {
    text: &'a str,
}

impl Component for Handwritten<'_> {
    fn render_component(
        &self,
        f: &mut (dyn fmt::Write + '_),
        attrs: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
        children: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
    ) -> fmt::Result {
        f.write_str("<button")?;
        attrs(f)?;
        f.write_str(">")?;
        self.text.render_escaped(f)?;
        children(f)?;
        f.write_str("</button>")
    }
}

component! {
    fn Migrated(text: &str, disabled: bool = false) {
        button(disabled?: disabled, #attrs) { (text) #children }
    }
}

#[test]
fn handwritten_direct_rendering_and_macro_migration_are_equivalent() {
    let source = String::from("<Save>");
    let text = source.as_str();
    let mut direct = String::new();
    Handwritten { text }
        .render_component(
            &mut direct,
            |f| f.write_str(" data-test=\"custom\""),
            |f| "!".render_escaped(f),
        )
        .unwrap();
    let migrated = html! { @Migrated(text; data_test: "custom") { "!" } }.to_html();
    assert_eq!(migrated, direct.as_str());
    assert_eq!(
        direct,
        "<button data-test=\"custom\">&lt;Save&gt;!</button>"
    );
}
