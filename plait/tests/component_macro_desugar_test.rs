use plait::{ClassPart, IntoHtml, classes, component, html, render};

// Anonymous lifetime: &str desugared to &'plait_0 str
component! {
    pub fn AnonymousLifetimeButton(label: &str) {
        button {
            (label)
        }
    }
}

#[test]
fn test_anonymous_lifetime() {
    let html = render(html! {
        @AnonymousLifetimeButton(label: "Click me") {}
    });
    assert_eq!(html, "<button>Click me</button>");
}

// impl Trait desugared to generic type parameter
component! {
    pub fn ImplTraitButton(class: impl ClassPart) {
        button(class: classes!("btn", class)) {
            #children
        }
    }
}

#[test]
fn test_impl_trait() {
    let html = render(html! {
        @ImplTraitButton(class: "primary") {
            "Click"
        }
    });
    assert_eq!(html, "<button class=\"btn primary\">Click</button>");
}

#[test]
fn test_impl_trait_with_option() {
    let html = render(html! {
        @ImplTraitButton(class: Some("primary")) {
            "Click"
        }
    });
    assert_eq!(html, "<button class=\"btn primary\">Click</button>");
}

// Both anonymous lifetime and impl Trait together
component! {
    pub fn CombinedButton(id: &str, class: impl ClassPart) {
        button(id: id, class: classes!("btn", class), #attrs) {
            #children
        }
    }
}

#[test]
fn test_combined_lifetime_and_impl_trait() {
    let html = render(html! {
        @CombinedButton(id: "btn1", class: "primary") {
            "Click"
        }
    });
    assert_eq!(
        html,
        "<button id=\"btn1\" class=\"btn primary\">Click</button>"
    );
}

// Mixed explicit and anonymous lifetimes
component! {
    pub fn MixedLifetimes<'a>(explicit: &'a str, anonymous: &str) {
        div {
            span { (explicit) }
            span { (anonymous) }
        }
    }
}

#[test]
fn test_mixed_lifetimes() {
    let html = render(html! {
        @MixedLifetimes(explicit: "hello", anonymous: "world") {}
    });
    assert_eq!(html, "<div><span>hello</span><span>world</span></div>");
}

// Multiple impl Trait parameters
component! {
    pub fn MultiImplTrait(class1: impl ClassPart, class2: impl ClassPart) {
        div(class: classes!(class1, class2)) {
            #children
        }
    }
}

#[test]
fn test_multiple_impl_traits() {
    let html = render(html! {
        @MultiImplTrait(class1: "foo", class2: "bar") {
            "content"
        }
    });
    assert_eq!(html, "<div class=\"foo bar\">content</div>");
}

// Option<&str> with anonymous lifetime
component! {
    pub fn OptionalRefButton(class: Option<&str>) {
        button(class: classes!("btn", class)) {
            #children
        }
    }
}

#[test]
fn test_option_with_anonymous_lifetime() {
    let html = render(html! {
        @OptionalRefButton(class: Some("primary")) {
            "Click"
        }
    });
    assert_eq!(html, "<button class=\"btn primary\">Click</button>");
}

#[test]
fn test_option_with_anonymous_lifetime_none() {
    let html = render(html! {
        @OptionalRefButton(class: None) {
            "Click"
        }
    });
    assert_eq!(html, "<button class=\"btn\">Click</button>");
}

// impl Trait with multiple bounds
component! {
    pub fn ImplMultiBound(content: impl IntoHtml + Send) {
        div {
            (content)
        }
    }
}

#[test]
fn test_impl_trait_multiple_bounds() {
    let html = render(html! {
        @ImplMultiBound(content: html! { span { "inner" } }) {}
    });
    assert_eq!(html, "<div><span>inner</span></div>");
}

// Reference to impl Trait: &impl Display desugars to &'plait_0 P0
component! {
    pub fn RefImplTrait(label: &impl core::fmt::Display) {
        span {
            (label)
        }
    }
}

#[test]
fn test_ref_impl_trait() {
    let html = render(html! {
        @RefImplTrait(label: &"hello") {}
    });
    assert_eq!(html, "<span>hello</span>");
}

#[test]
fn test_ref_impl_trait_with_number() {
    let html = render(html! {
        @RefImplTrait(label: &42) {}
    });
    assert_eq!(html, "<span>42</span>");
}

// Mixed explicit generics with impl Trait and anonymous lifetimes
component! {
    pub fn FullMix<T>(header: T, label: &str, class: impl ClassPart) where T: IntoHtml {
        div(class: classes!("card", class)) {
            h1 { (header) }
            span { (label) }
            #children
        }
    }
}

#[test]
fn test_full_mix() {
    let html = render(html! {
        @FullMix(header: html! { "Title" }, label: "subtitle", class: "primary") {
            "body"
        }
    });
    assert_eq!(
        html,
        "<div class=\"card primary\"><h1>Title</h1><span>subtitle</span>body</div>"
    );
}

// Kitchen sink: user-provided lifetime, user-provided type param, anonymous lifetime,
// impl Trait, concrete type, and a where clause — all in one component.
component! {
    pub fn KitchenSink<'a, T>(
        explicit_ref: &'a str,
        anonymous_ref: &str,
        concrete: bool,
        count: u32,
        header: T,
        class: impl ClassPart,
        extra_class: impl ClassPart,
    ) where T: IntoHtml {
        div(class: classes!("card", class, extra_class), #attrs) {
            h1 { (header) }
            span(class: "label") { (explicit_ref) " " (anonymous_ref) }
            if concrete {
                span(class: "badge") { (count) }
            }
            #children
        }
    }
}

#[test]
fn test_kitchen_sink_all_present() {
    let html = render(html! {
        @KitchenSink(
            explicit_ref: "hello",
            anonymous_ref: "world",
            concrete: true,
            count: 5,
            header: html! { strong { "Title" } },
            class: "primary",
            extra_class: Some("large");
            id: "card-1"
        ) {
            p { "body content" }
        }
    });
    assert_eq!(
        html,
        "<div class=\"card primary large\" id=\"card-1\">\
         <h1><strong>Title</strong></h1>\
         <span class=\"label\">hello world</span>\
         <span class=\"badge\">5</span>\
         <p>body content</p>\
         </div>"
    );
}

#[test]
fn test_kitchen_sink_concrete_false() {
    let html = render(html! {
        @KitchenSink(
            explicit_ref: "a",
            anonymous_ref: "b",
            concrete: false,
            count: 0,
            header: "Simple",
            class: "secondary",
            extra_class: None::<&str>
        ) {}
    });
    assert_eq!(
        html,
        "<div class=\"card secondary\">\
         <h1>Simple</h1>\
         <span class=\"label\">a b</span>\
         </div>"
    );
}

// Explicit lifetime + anonymous lifetime + impl Trait + concrete, no where clause
component! {
    pub fn NavLink<'a>(href: &'a str, label: &str, class: impl ClassPart, active: bool) {
        a(href: href, class: classes!("nav-link", class, if active { "active" } else { "" })) {
            (label)
        }
    }
}

#[test]
fn test_nav_link_active() {
    let html = render(html! {
        @NavLink(href: "/home", label: "Home", class: "primary", active: true) {}
    });
    assert_eq!(
        html,
        "<a href=\"/home\" class=\"nav-link primary active\">Home</a>"
    );
}

#[test]
fn test_nav_link_inactive() {
    let html = render(html! {
        @NavLink(href: "/about", label: "About", class: None::<&str>, active: false) {}
    });
    assert_eq!(html, "<a href=\"/about\" class=\"nav-link\">About</a>");
}

// Two user-provided type params + anonymous lifetime + impl Trait
component! {
    pub fn DataCard<H, F>(
        header: H,
        footer: F,
        label: &str,
        class: impl ClassPart,
    ) where H: IntoHtml, F: IntoHtml {
        div(class: classes!("data-card", class)) {
            div(class: "header") { (header) }
            div(class: "body") { (label) #children }
            div(class: "footer") { (footer) }
        }
    }
}

#[test]
fn test_data_card() {
    let html = render(html! {
        @DataCard(
            header: html! { h2 { "Stats" } },
            footer: html! { small { "Updated today" } },
            label: "Count: ",
            class: "highlighted"
        ) {
            strong { "42" }
        }
    });
    assert_eq!(
        html,
        "<div class=\"data-card highlighted\">\
         <div class=\"header\"><h2>Stats</h2></div>\
         <div class=\"body\">Count: <strong>42</strong></div>\
         <div class=\"footer\"><small>Updated today</small></div>\
         </div>"
    );
}

// Multiple anonymous lifetimes + multiple impl Traits + concrete types interleaved
component! {
    pub fn FormField(
        name: &str,
        label_text: &str,
        field_type: &str,
        required: bool,
        class: impl ClassPart,
        label_class: impl ClassPart,
    ) {
        div(class: classes!("form-field", class)) {
            label(class: classes!("form-label", label_class)) { (label_text) }
            input(type: field_type, name: name, required?: required);
        }
    }
}

#[test]
fn test_form_field_required() {
    let html = render(html! {
        @FormField(
            name: "email",
            label_text: "Email",
            field_type: "email",
            required: true,
            class: "mb-4",
            label_class: "font-bold"
        ) {}
    });
    assert_eq!(
        html,
        "<div class=\"form-field mb-4\">\
         <label class=\"form-label font-bold\">Email</label>\
         <input type=\"email\" name=\"email\" required>\
         </div>"
    );
}

#[test]
fn test_form_field_optional() {
    let html = render(html! {
        @FormField(
            name: "bio",
            label_text: "Bio",
            field_type: "text",
            required: false,
            class: None::<&str>,
            label_class: None::<&str>
        ) {}
    });
    assert_eq!(
        html,
        "<div class=\"form-field\">\
         <label class=\"form-label\">Bio</label>\
         <input type=\"text\" name=\"bio\">\
         </div>"
    );
}
