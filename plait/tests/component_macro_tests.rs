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
