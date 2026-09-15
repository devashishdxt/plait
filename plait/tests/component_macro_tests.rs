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
