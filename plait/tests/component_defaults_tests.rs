use plait::{Component, ToHtml, component, html};

#[path = "support/default_components.rs"]
mod fixtures;
use fixtures::{Appearance, Button, ButtonKind, Dialog, Legacy, LinkButton, TextField};

#[test]
fn p01_minimal_button() {
    assert_eq!(
        html! { @Button { "Cancel" } }.to_html(),
        "<button type=\"button\" class=\"solid\">Cancel</button>"
    );
    assert_eq!(
        html! { @Button() {} }.to_html(),
        "<button type=\"button\" class=\"solid\"></button>"
    );
}

#[test]
fn p02_independent_defaults() {
    assert_eq!(
        html! { @Button(kind: ButtonKind::Submit) {} }.to_html(),
        "<button type=\"submit\" class=\"solid\"></button>"
    );
    assert_eq!(
        html! { @Button(appearance: Appearance::Outline) {} }.to_html(),
        "<button type=\"button\" class=\"outline\"></button>"
    );
    assert_eq!(
        html! { @Button(disabled: true) {} }.to_html(),
        "<button type=\"button\" class=\"solid\" disabled></button>"
    );
    assert_eq!(
        html! { @Button(class: "layout") {} }.to_html(),
        "<button type=\"button\" class=\"solid layout\"></button>"
    );
    assert_eq!(
        html! { @Button(class: "layout", disabled: true,
        appearance: Appearance::Outline, kind: ButtonKind::Submit) { "Save" } }
        .to_html(),
        "<button type=\"submit\" class=\"outline layout\" disabled>Save</button>"
    );
}

#[test]
fn p03_minimal_field() {
    assert_eq!(
        html! { @TextField(id: "email", name: "email", label: "Email") {} }.to_html(),
        "<label for=\"email\">Email</label><input id=\"email\" name=\"email\" type=\"text\" value=\"\">"
    );
}

#[test]
fn p04_borrowed_text() {
    let id = String::from("email");
    let name = String::from("contact");
    let label = String::from("Email");
    let value = String::from("invalid");
    let description = Some(String::from("Work address"));
    let mut error = Some(String::from("Invalid address"));
    let (id, name, label, value) = (id.as_str(), name.as_str(), label.as_str(), value.as_str());
    let description = description.as_deref();
    let render = |error: Option<&str>| {
        html! {
            @TextField(id, name, label, value, description, error) {}
        }
        .to_html()
    };
    assert_eq!(
        render(error.as_deref()),
        "<label for=\"email\">Email</label><input id=\"email\" name=\"contact\" type=\"text\" value=\"invalid\"><p class=\"description\">Work address</p><p class=\"error\">Invalid address</p>"
    );
    error = None;
    assert!(!render(error.as_deref()).contains("class=\"error\""));
}

#[test]
fn p05_optional_text_combinations() {
    let base = "<label for=\"x\">X</label><input id=\"x\" name=\"x\" type=\"text\" value=\"\">";
    assert_eq!(
        html! { @TextField(id: "x", name: "x", label: "X", description: Some("Help")) {} }
            .to_html(),
        format!("{base}<p class=\"description\">Help</p>").as_str()
    );
    assert_eq!(
        html! { @TextField(id: "x", name: "x", label: "X", error: Some("Error")) {} }.to_html(),
        format!("{base}<p class=\"error\">Error</p>").as_str()
    );
    assert_eq!(html! { @TextField(id: "x", name: "x", label: "X", description: Some("Help"), error: Some("Error")) {} }.to_html(),
        format!("{base}<p class=\"description\">Help</p><p class=\"error\">Error</p>").as_str());
    assert_eq!(html! { @TextField(id: "x", name: "x", label: "X", description: Some("Help"), error: None) {} }.to_html(),
        format!("{base}<p class=\"description\">Help</p>").as_str());
    assert_eq!(
        html! { @TextField(id: "x", name: "x", label: "X", description: None, error: None) {} }
            .to_html(),
        base
    );
}

#[test]
fn p06_minimal_dialog() {
    let title = String::from("Edit");
    let title = title.as_str();
    assert_eq!(
        html! { @Dialog(id: "edit", title: html! { (title) }) { p { "Body" } } }.to_html(),
        "<dialog id=\"edit\" aria-labelledby=\"edit-title\"><h2 id=\"edit-title\">Edit</h2><div id=\"edit-body\"><p>Body</p></div></dialog>"
    );
}

#[test]
fn p07_independent_fragments() {
    let help_text = String::from("Help");
    let action_text = String::from("Save");
    let (help_text, action_text) = (help_text.as_str(), action_text.as_str());
    let description = html! { p { (help_text) } };
    let actions = html! { @Button { (action_text) } };
    let (description, actions) = (&description, &actions);
    let help =
        html! { @Dialog(id: "x", title: html! { "Title" }, description: Some(description)) {} }
            .to_html();
    assert!(help.contains("aria-describedby=\"x-description\""));
    assert!(help.contains("<div id=\"x-description\"><p>Help</p></div>"));
    assert!(!help.contains("<footer>"));
    let footer =
        html! { @Dialog(id: "x", title: html! { "Title" }, actions: Some(actions)) {} }.to_html();
    assert!(
        footer.contains("<footer><button type=\"button\" class=\"solid\">Save</button></footer>")
    );
    assert!(!footer.contains("x-description"));
    let both = html! { @Dialog(id: "x", title: html! { strong { "Title" } },
    description: Some(description), actions: Some(actions)) { "Body" } }
    .to_html();
    assert_eq!(
        both,
        "<dialog id=\"x\" aria-labelledby=\"x-title\" aria-describedby=\"x-description\"><h2 id=\"x-title\"><strong>Title</strong></h2><div id=\"x-description\"><p>Help</p></div><div id=\"x-body\">Body</div><footer><button type=\"button\" class=\"solid\">Save</button></footer></dialog>"
    );
    // Temporary fragment borrows and explicit untyped None also need no annotations.
    assert!(
        html! { @Dialog(id: "x", title: html! { "Title" },
        description: Some(&html! { em { (help_text) } }), actions: None) {} }
        .to_html()
        .contains("<em>Help</em>")
    );
}

#[test]
fn p08_legacy_calls() {
    let label = String::from("Label");
    let note = Some("Note");
    let count = 2;
    assert_eq!(html! { @Legacy(label: label.as_str(), note, content: html! { b { "Content" } }, count; data_test: "yes") {
        "Children"
    } }.to_html(), "<section data-count=\"2\" data-test=\"yes\">LabelNote<b>Content</b>Children</section>");
    // Direct struct construction and the rendering trait remain available.
    let direct = Button {
        kind: ButtonKind::Button,
        appearance: Appearance::Solid,
        disabled: false,
        class: "",
    };
    let mut out = String::new();
    direct
        .render_component(&mut out, |_| Ok(()), |f| f.write_str("Direct"))
        .unwrap();
    assert_eq!(
        out,
        "<button type=\"button\" class=\"solid\">Direct</button>"
    );
}

#[test]
fn p09_link_destination() {
    assert_eq!(
        html! { @LinkButton(href: "/home"; data_test: "link") { "Home" } }.to_html(),
        "<a href=\"/home\" class=\"solid\" data-test=\"link\">Home</a>"
    );
}

#[test]
fn p10_escaped_borrowed_content() {
    let value = String::from("<b>&\"'");
    let value = value.as_str();
    let error = Some(String::from("<script>bad</script>"));
    assert_eq!(
        html! { @TextField(id: "x", name: "x", label: "X", value, error: error.as_deref();
        hx_get: "/check?a=1&b=2", hx_trigger: "change", autocomplete: "email", required) {} }
        .to_html(),
        "<label for=\"x\">X</label><input id=\"x\" name=\"x\" type=\"text\" value=\"&lt;b&gt;&amp;&quot;&#39;\" hx-get=\"/check?a=1&amp;b=2\" hx-trigger=\"change\" autocomplete=\"email\" required><p class=\"error\">&lt;script&gt;bad&lt;/script&gt;</p>"
    );
    let fragment = html! { p { (value) } };
    assert!(
        html! { @Dialog(id: "x", title: html! { (value) }, description: Some(&fragment)) {} }
            .to_html()
            .contains("<div id=\"x-description\"><p>&lt;b&gt;&amp;&quot;&#39;</p></div>")
    );
}

#[test]
fn defaults_are_lazy_and_explicit_none_overrides_a_nonempty_default() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn next() -> usize {
        CALLS.fetch_add(1, Ordering::SeqCst)
    }
    component! {
        fn Count(value: usize = next(), note: Option<&str> = Some("default")) {
            span { (value) (note) }
        }
    }
    let page = html! { @Count {} };
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(page.to_html(), "<span>0default</span>");
    assert_eq!(page.to_html(), "<span>1default</span>");
    assert_eq!(
        html! { @Count(value: 42, note: None) {} }.to_html(),
        "<span>42</span>"
    );
    assert_eq!(CALLS.load(Ordering::SeqCst), 2);
}

#[test]
fn generic_defaults_and_const_generics() {
    component! {
        fn Generic<'a, T, const N: usize>(values: &'a [T; N], suffix: T = T::default())
        where T: Default + plait::RenderEscaped {
            for value in values.iter() { (value) }
            (suffix)
        }
    }
    assert_eq!(html! { @Generic(values: &[1_u32, 2]) {} }.to_html(), "120");
    assert_eq!(
        html! { @Generic(values: &[1_u32, 2], suffix: 3) {} }.to_html(),
        "123"
    );
}

#[test]
fn borrowed_fragment_can_be_returned_without_moving_owners() {
    fn field<'a>(value: &'a str, error: Option<&'a str>) -> impl plait::PartialHtml + ToHtml + 'a {
        html! { @TextField(id: "x", name: "x", label: "X", value, error) {} }
    }
    let value = String::from("Value");
    let error = Some(String::from("Error"));
    let page = field(value.as_str(), error.as_deref());
    assert!(page.to_html().contains("value=\"Value\""));
    assert!(page.to_html().contains("<p class=\"error\">Error</p>"));
    assert_eq!(value, "Value");
    assert_eq!(error.as_deref(), Some("Error"));
}

#[test]
fn nested_defaulted_components_forward_attributes_and_children() {
    component! { fn Wrapper(class: &str = "") {
        @Button(class; #attrs) { #children }
    } }
    assert_eq!(
        html! { @Wrapper(; data_test: "nested") { "Body" } }.to_html(),
        "<button type=\"button\" class=\"solid\" data-test=\"nested\">Body</button>"
    );
}

#[test]
fn generated_state_names_do_not_collide_and_generic_defaults_still_parse() {
    component! {
        fn Named<__PlaitState0 = u32>(value: __PlaitState0, suffix: &str = "!")
        where __PlaitState0: plait::RenderEscaped {
            (value) (suffix)
        }
    }
    assert_eq!(html! { @Named(value: 1_u32) {} }.to_html(), "1!");
    assert_eq!(
        html! { @Named::<&str>(value: "one", suffix: "?") {} }.to_html(),
        "one?"
    );
}

#[test]
fn zero_props_and_raw_identifiers() {
    component! { fn Empty { br; } }
    component! { fn Raw(r#type: &str = "text", r#__plait_marker: &str = "marker") {
        input(type: r#type, data_marker: r#__plait_marker);
    } }
    assert_eq!(html! { @Empty {} }.to_html(), "<br>");
    assert_eq!(
        html! { @Raw(r#type: "email") {} }.to_html(),
        "<input type=\"email\" data-marker=\"marker\">"
    );
}
