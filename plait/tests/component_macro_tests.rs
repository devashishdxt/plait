use plait::{Class, RenderEscaped, ToHtml, classes, component, html};

component! {
    pub fn Button<'a>(class: Option<&'a str>) {
        button(class: classes!("btn", class), #attrs) {
            #children
        }
    }
}

component! {
    pub fn Card<T>(title: T) where T: RenderEscaped {
        div(class: "card") {
            h1 { (title) }
            @Button(class: "btn-primary".into(); #attrs) {
                #children
            }
        }
    }
}

#[test]
fn test_button() {
    let disabled = false;

    let html = html! {
        @Button(class: None; id: "btn1", disabled?: disabled) {
            "Click me"
        }
    };

    assert_eq!(
        html.to_html(),
        "<button class=\"btn\" id=\"btn1\">Click me</button>"
    );
}

#[test]
fn test_card() {
    let disabled = true;

    let html = html! {
        @Card(title: html! { span { "My card" } }; disabled?: disabled) {
            "Click me"
        }
    };

    assert_eq!(
        html.to_html(),
        "<div class=\"card\"><h1><span>My card</span></h1><button class=\"btn btn-primary\" disabled>Click me</button></div>"
    );
}

// --- Shorthand argument tests ---

component! {
    pub fn Link(href: &str, class: impl Class) {
        a(href: href, class: classes!(class), #attrs) {
            #children
        }
    }
}

component! {
    pub fn Greeting(name: &str) {
        span { "Hello, " (name) "!" }
    }
}

component! {
    pub fn UserCard(name: &str, role: &str) {
        div(class: "user-card") {
            span(class: "name") { (name) }
            span(class: "role") { (role) }
        }
    }
}

#[test]
fn test_shorthand_single_field() {
    let name = "Alice";

    let html = html! {
        @Greeting(name) {}
    };

    assert_eq!(html.to_html(), "<span>Hello, Alice!</span>");
}

#[test]
fn test_shorthand_multiple_fields() {
    let name = "Alice";
    let role = "Admin";

    let html = html! {
        @UserCard(name, role) {}
    };

    assert_eq!(
        html.to_html(),
        "<div class=\"user-card\"><span class=\"name\">Alice</span><span class=\"role\">Admin</span></div>"
    );
}

#[test]
fn test_shorthand_mixed_with_explicit() {
    let href = "https://example.com/";

    let html = html! {
        @Link(href, class: Some("link")) {
            "My Link"
        }
    };

    assert_eq!(
        html.to_html(),
        "<a href=\"https://example.com/\" class=\"link\">My Link</a>"
    );
}

#[test]
fn test_shorthand_with_attributes() {
    let href = "https://example.com/";
    let class: Option<&str> = None;

    let html = html! {
        @Link(href, class; id: "my-link") {
            "Click"
        }
    };

    assert_eq!(
        html.to_html(),
        "<a href=\"https://example.com/\" class=\"\" id=\"my-link\">Click</a>"
    );
}

#[test]
fn test_shorthand_equivalent_to_explicit() {
    let name = "Bob";
    let role = "User";

    let shorthand = html! {
        @UserCard(name, role) {}
    };

    let explicit = html! {
        @UserCard(name: name, role: role) {}
    };

    assert_eq!(shorthand.to_html(), explicit.to_html());
}

#[test]
fn test_shorthand_with_ref_lifetime() {
    let label = "Click me";

    let html = html! {
        @Button(class: None) {
            (label)
        }
    };

    // Also test shorthand with Option field
    let class: Option<&str> = Some("primary");

    let html2 = html! {
        @Button(class) {
            "Submit"
        }
    };

    assert_eq!(html.to_html(), "<button class=\"btn\">Click me</button>");
    assert_eq!(
        html2.to_html(),
        "<button class=\"btn primary\">Submit</button>"
    );
}

mod default_components {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static EVENTS: RefCell<Vec<&'static str>> = const { RefCell::new(Vec::new()) };
    }

    pub fn record(event: &'static str) -> &'static str {
        EVENTS.with(|events| events.borrow_mut().push(event));
        event
    }

    pub fn take_events() -> Vec<&'static str> {
        EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut()))
    }

    component! {
        pub fn ActionButton(
            kind: &str = "button",
            disabled: bool = false,
            tooltip: Option<&str> = Some("Save changes"),
        ) {
            button(type: kind, disabled?: disabled, title?: tooltip) { #children }
        }
    }

    component! {
        pub fn ArticlePreview(
            title: &str = record("Untitled"),
            summary: &str = record("No summary"),
            author: Option<&str> = Some(record("Anonymous")),
            link: &str = record("/articles"),
        ) {
            article {
                h2 { a(href: link) { (title) } }
                p { (summary) }
                if let Some(author) = author { footer { (author) } }
            }
        }
    }

    component! { pub fn Divider() { hr; } }

    component! {
        pub fn Panel() {
            (html! { section(#attrs) { #children } })
        }
    }
}

use default_components::ActionButton as SaveButton;

#[test]
fn defaults_are_independently_overridable() {
    let disabled = true;
    assert_eq!(
        html! {
            @default_components::ActionButton() { "Save" }
            @SaveButton(kind: "submit") { "Publish" }
            @SaveButton(disabled) { "Saving" }
            @SaveButton(disabled: true, kind: "reset", tooltip: None) { "Reset" }
            @default_components::Divider {}
            @default_components::Divider() {}
        }
        .to_html(),
        concat!(
            "<button type=\"button\" title=\"Save changes\">Save</button>",
            "<button type=\"submit\" title=\"Save changes\">Publish</button>",
            "<button type=\"button\" disabled title=\"Save changes\">Saving</button>",
            "<button type=\"reset\" disabled>Reset</button><hr><hr>",
        )
    );
}

#[test]
fn defaults_run_lazily_in_declaration_scope_and_order() {
    use default_components::take_events;
    take_events();
    // This caller-local name must not capture the declaration's `record` helper.
    let record = || panic!("caller-local helper must not run");
    let _ = &record;
    let page = html! {
        @default_components::ArticlePreview(
            link: default_components::record("/draft"),
            summary: default_components::record("Draft summary"),
        ) {}
    };
    assert!(take_events().is_empty());
    for _ in 0..2 {
        assert_eq!(
            page.to_html(),
            "<article><h2><a href=\"/draft\">Untitled</a></h2><p>Draft summary</p><footer>Anonymous</footer></article>"
        );
        assert_eq!(
            take_events(),
            ["/draft", "Draft summary", "Untitled", "Anonymous"]
        );
    }
    let override_all = html! {
        @default_components::ArticlePreview(
            author: None,
            summary: default_components::record("Published summary"),
            title: default_components::record("Launch"),
            link: default_components::record("/launch"),
        ) {}
    };
    assert!(take_events().is_empty());
    assert_eq!(
        override_all.to_html(),
        "<article><h2><a href=\"/launch\">Launch</a></h2><p>Published summary</p></article>"
    );
    assert_eq!(take_events(), ["Published summary", "Launch", "/launch"]);
}

#[test]
fn nested_fragments_forward_attributes_and_children() {
    assert_eq!(
        html! {
            @default_components::Panel(; class: "panel") { p { "Welcome" } }
        }
        .to_html(),
        "<section class=\"panel\"><p>Welcome</p></section>"
    );
}

mod reserved_attributes {
    use super::*;

    component! {
        #[reserve_attrs(type, class, aria_disabled, "x-on:click",)]
        #[reserve_attrs()]
        #[reserve_attrs(type)]
        fn ActionButton(class: &str = "btn") {
            button(type: "button", class: class, #attrs) { #children }
        }
    }

    component! {
        fn ToolbarButton() {
            // Repeat the spread deliberately to test ordering and evaluation.
            @ActionButton(; data_action: "save", #attrs, data_tracking: "toolbar", #attrs) { #children }
        }
    }

    component! {
        fn Toolbar(visible: bool = true) {
            (html! { if *visible { @ToolbarButton(; #attrs) { #children } } })
        }
    }

    #[test]
    fn toolbar_attributes_keep_order_and_repeated_spreads() {
        assert_eq!(
            html! { @ToolbarButton(; id: "save") { "Save" } }.to_html(),
            "<button type=\"button\" class=\"btn\" data-action=\"save\" id=\"save\" data-tracking=\"toolbar\" id=\"save\">Save</button>"
        );
        assert_eq!(
            html! { @Toolbar(; id: "save") { "Save" } }.to_html(),
            "<button type=\"button\" class=\"btn\" data-action=\"save\" id=\"save\" data-tracking=\"toolbar\" id=\"save\">Save</button>"
        );
    }

    // Deliberately use protocol-like names to test macro hygiene.
    component! {
        fn AttributePreview<__PlaitAttributes>(attrs: __PlaitAttributes, children: &str)
        where
            __PlaitAttributes: AsRef<str>,
        {
            if true {
                for _ in 0..1 {
                    match true {
                        true => @ActionButton(; #attrs) { (attrs.as_ref()) (children) #children },
                        false => {},
                    }
                }
            }
        }
    }

    #[test]
    fn inner_buffers_preserve_attribute_hygiene() {
        let __plait_attrs = "id";
        let __plait_component = "text";
        let __plait_children = "child";
        type __PlaitAttributeNames = &'static str;
        let __plait_attribute_names = "name";
        assert_eq!(
            html! {
                @AttributePreview(attrs: __plait_component, children: __plait_children;
                    id: __plait_attrs, data_name: {
                        let value: __PlaitAttributeNames = __plait_attribute_names;
                        value
                    }) { "forwarded" }
            }
            .to_html(),
            "<button type=\"button\" class=\"btn\" id=\"id\" data-name=\"name\">textchildforwarded</button>"
        );
    }

    #[test]
    fn component_references_keep_the_attribute_bundle() {
        let component = ActionButton::__plait_props().__plait_resolve();
        let mut output = String::new();
        let attributes = plait::__attrs::Bundle::new((), |writer| writer.write_str(" id=\"save\""));
        plait::Component::render_component(&&component, &mut output, &attributes, |writer| {
            writer.write_str("Save")
        })
        .unwrap();
        assert_eq!(
            output,
            "<button type=\"button\" class=\"btn\" id=\"save\">Save</button>"
        );
    }

    component! {
        fn ButtonGroup() {
            div(role: "group", #attrs) { @ActionButton(class: "primary") { #children } }
        }
    }

    component! {
        fn SubmitButton() { button(type: "submit", #attrs) { #children } }
    }

    component! {
        #[reserve_attrs(type)]
        fn SaveButton() { button(type: "button") { #children } }
    }

    #[test]
    fn reservations_only_apply_to_the_receiving_component() {
        assert_eq!(
            html! { @ButtonGroup(; type: "submit", class: "toolbar") { "Save" } }.to_html(),
            "<div role=\"group\" type=\"submit\" class=\"toolbar\"><button type=\"button\" class=\"primary\">Save</button></div>"
        );
        assert_eq!(
            html! { @SubmitButton(; type: "button", class: "primary") { "Save" } }.to_html(),
            "<button type=\"submit\" type=\"button\" class=\"primary\">Save</button>"
        );
    }

    #[test]
    fn attributes_stay_lazy_and_preserve_evaluation_order() {
        use std::cell::RefCell;
        fn track<T>(events: &RefCell<Vec<&'static str>>, event: &'static str, value: T) -> T {
            events.borrow_mut().push(event);
            value
        }
        let events = RefCell::new(Vec::new());
        let log = &events;
        let page = html! {
            @ToolbarButton(;
                data_escaped: (track(log, "escaped", "<&")),
                data_raw: #(track(log, "raw", "<&")),
                title?: track(log, "some", Some("<&")),
                hidden?: track(log, "boolean", true),
                data_none?: track(log, "none", None::<&str>),
                data_false?: false,
            ) { (track(log, "child", "<&")) }
        };
        assert!(events.borrow().is_empty());
        for _ in 0..2 {
            assert_eq!(
                page.to_html(),
                concat!(
                    "<button type=\"button\" class=\"btn\" data-action=\"save\"",
                    " data-escaped=\"&lt;&amp;\" data-raw=\"<&\" title=\"&lt;&amp;\" hidden",
                    " data-tracking=\"toolbar\"",
                    " data-escaped=\"&lt;&amp;\" data-raw=\"<&\" title=\"&lt;&amp;\" hidden",
                    ">&lt;&amp;</button>",
                )
            );
            assert_eq!(
                std::mem::take(&mut *events.borrow_mut()),
                [
                    "escaped", "raw", "some", "boolean", "none", "escaped", "raw", "some",
                    "boolean", "none", "child",
                ]
            );
        }
        let ignored = html! { @SaveButton(; id: track(log, "unused", "save")) { "Save" } };
        assert_eq!(ignored.to_html(), "<button type=\"button\">Save</button>");
        assert!(events.borrow().is_empty());
    }

    #[test]
    fn attribute_output_spelling_is_unchanged() {
        assert_eq!(
            html! { @ActionButton(; "DATA-ID": "save", aria_label: "Save", "@click": "save()") {} }
                .to_html(),
            "<button type=\"button\" class=\"btn\" DATA-ID=\"save\" aria-label=\"Save\" @click=\"save()\"></button>"
        );
    }
}
