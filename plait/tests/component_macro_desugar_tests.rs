use plait::{Class, EmptyHtml, RenderEscaped, ToHtml, classes, component, html};

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
    let html = html! {
        @AnonymousLifetimeButton(label: "Click me") {}
    };
    assert_eq!(html.to_html(), "<button>Click me</button>");
}

// impl Trait desugared to generic type parameter
component! {
    pub fn ImplTraitButton(class: impl Class) {
        button(class: classes!("btn", class)) {
            #children
        }
    }
}

#[test]
fn test_impl_trait() {
    let html = html! {
        @ImplTraitButton(class: "primary") {
            "Click"
        }
    };
    assert_eq!(
        html.to_html(),
        "<button class=\"btn primary\">Click</button>"
    );
}

#[test]
fn test_impl_trait_with_option() {
    let html = html! {
        @ImplTraitButton(class: Some("primary")) {
            "Click"
        }
    };
    assert_eq!(
        html.to_html(),
        "<button class=\"btn primary\">Click</button>"
    );
}

// Both anonymous lifetime and impl Trait together
component! {
    pub fn CombinedButton(id: &str, class: impl Class) {
        button(id: id, class: classes!("btn", class), #attrs) {
            #children
        }
    }
}

#[test]
fn test_combined_lifetime_and_impl_trait() {
    let html = html! {
        @CombinedButton(id: "btn1", class: "primary") {
            "Click"
        }
    };
    assert_eq!(
        html.to_html(),
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
    let html = html! {
        @MixedLifetimes(explicit: "hello", anonymous: "world") {}
    };
    assert_eq!(
        html.to_html(),
        "<div><span>hello</span><span>world</span></div>"
    );
}

// Multiple impl Trait parameters
component! {
    pub fn MultiImplTrait(class1: impl Class, class2: impl Class) {
        div(class: classes!(class1, class2)) {
            #children
        }
    }
}

#[test]
fn test_multiple_impl_traits() {
    let html = html! {
        @MultiImplTrait(class1: "foo", class2: "bar") {
            "content"
        }
    };
    assert_eq!(html.to_html(), "<div class=\"foo bar\">content</div>");
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
    let html = html! {
        @OptionalRefButton(class: Some("primary")) {
            "Click"
        }
    };
    assert_eq!(
        html.to_html(),
        "<button class=\"btn primary\">Click</button>"
    );
}

#[test]
fn test_option_with_anonymous_lifetime_none() {
    let html = html! {
        @OptionalRefButton(class: None) {
            "Click"
        }
    };
    assert_eq!(html.to_html(), "<button class=\"btn\">Click</button>");
}

// impl Trait with multiple bounds
component! {
    pub fn ImplMultiBound(content: impl RenderEscaped + Send) {
        div {
            (content)
        }
    }
}

#[test]
fn test_impl_trait_multiple_bounds() {
    let html = html! {
        @ImplMultiBound(content: html! { span { "inner" } }) {}
    };
    assert_eq!(html.to_html(), "<div><span>inner</span></div>");
}

// Reference to impl Trait: &impl RenderEscaped desugars to &'plait_0 P0
component! {
    pub fn RefImplTrait(label: &impl ::plait::RenderEscaped) {
        span {
            (label)
        }
    }
}

#[test]
fn test_ref_impl_trait() {
    let html = html! {
        @RefImplTrait(label: &"hello") {}
    };
    assert_eq!(html.to_html(), "<span>hello</span>");
}

#[test]
fn test_ref_impl_trait_with_number() {
    let html = html! {
        @RefImplTrait(label: &42) {}
    };
    assert_eq!(html.to_html(), "<span>42</span>");
}

// Mixed explicit generics with impl Trait and anonymous lifetimes
component! {
    pub fn FullMix<T>(header: T, label: &str, class: impl Class) where T: RenderEscaped {
        div(class: classes!("card", class)) {
            h1 { (header) }
            span { (label) }
            #children
        }
    }
}

#[test]
fn test_full_mix() {
    let html = html! {
        @FullMix(header: html! { "Title" }, label: "subtitle", class: "primary") {
            "body"
        }
    };
    assert_eq!(
        html.to_html(),
        "<div class=\"card primary\"><h1>Title</h1><span>subtitle</span>body</div>"
    );
}

// Kitchen sink: user-provided lifetime, user-provided type param, anonymous lifetime,
// impl Trait, concrete type, and a where clause - all in one component.
component! {
    pub fn KitchenSink<'a, T>(
        explicit_ref: &'a str,
        anonymous_ref: &str,
        concrete: bool,
        count: u32,
        header: T,
        class: impl Class,
        extra_class: impl Class,
    ) where T: RenderEscaped {
        div(class: classes!("card", class, extra_class), #attrs) {
            h1 { (header) }
            span(class: "label") { (explicit_ref) " " (anonymous_ref) }
            if *concrete {
                span(class: "badge") { (count) }
            }
            #children
        }
    }
}

#[test]
fn test_kitchen_sink_all_present() {
    let html = html! {
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
    };
    assert_eq!(
        html.to_html(),
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
    let html = html! {
        @KitchenSink(
            explicit_ref: "a",
            anonymous_ref: "b",
            concrete: false,
            count: 0,
            header: html! { "Simple" },
            class: "secondary",
            extra_class: None::<&str>
        ) {}
    };
    assert_eq!(
        html.to_html(),
        "<div class=\"card secondary\">\
         <h1>Simple</h1>\
         <span class=\"label\">a b</span>\
         </div>"
    );
}

// Explicit lifetime + anonymous lifetime + impl Trait + concrete, no where clause
component! {
    pub fn NavLink<'a>(href: &'a str, label: &str, class: impl Class, active: bool) {
        a(href: href, class: classes!("nav-link", class, if *active { "active" } else { "" })) {
            (label)
        }
    }
}

#[test]
fn test_nav_link_active() {
    let html = html! {
        @NavLink(href: "/home", label: "Home", class: "primary", active: true) {}
    };
    assert_eq!(
        html.to_html(),
        "<a href=\"/home\" class=\"nav-link primary active\">Home</a>"
    );
}

#[test]
fn test_nav_link_inactive() {
    let html = html! {
        @NavLink(href: "/about", label: "About", class: None::<&str>, active: false) {}
    };
    assert_eq!(
        html.to_html(),
        "<a href=\"/about\" class=\"nav-link\">About</a>"
    );
}

// Two user-provided type params + anonymous lifetime + impl Trait
component! {
    pub fn DataCard<H, F>(
        header: H,
        footer: F,
        label: &str,
        class: impl Class,
    ) where H: RenderEscaped, F: RenderEscaped {
        div(class: &classes!("data-card", class)) {
            div(class: "header") { (header) }
            div(class: "body") { (label) #children }
            div(class: "footer") { (footer) }
        }
    }
}

#[test]
fn test_data_card() {
    let html = html! {
        @DataCard(
            header: html! { h2 { "Stats" } },
            footer: html! { small { "Updated today" } },
            label: "Count: ",
            class: "highlighted"
        ) {
            strong { "42" }
        }
    };
    assert_eq!(
        html.to_html(),
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
        class: impl Class,
        label_class: impl Class,
    ) {
        div(class: classes!("form-field", class)) {
            label(class: classes!("form-label", label_class)) { (label_text) }
            input(type: field_type, name: name, required?: required);
        }
    }
}

#[test]
fn test_form_field_required() {
    let html = html! {
        @FormField(
            name: "email",
            label_text: "Email",
            field_type: "email",
            required: true,
            class: "mb-4",
            label_class: "font-bold"
        ) {}
    };
    assert_eq!(
        html.to_html(),
        "<div class=\"form-field mb-4\">\
         <label class=\"form-label font-bold\">Email</label>\
         <input type=\"email\" name=\"email\" required>\
         </div>"
    );
}

#[test]
fn test_form_field_optional() {
    let html = html! {
        @FormField(
            name: "bio",
            label_text: "Bio",
            field_type: "text",
            required: false,
            class: None::<&str>,
            label_class: None::<&str>
        ) {}
    };
    assert_eq!(
        html.to_html(),
        "<div class=\"form-field\">\
         <label class=\"form-label\">Bio</label>\
         <input type=\"text\" name=\"bio\">\
         </div>"
    );
}

component! {
    pub fn ArticleCard(
        title: impl AsRef<str> + Send = "Untitled",
        body: impl plait::PartialHtml = html! { p { "No content yet" } },
        footer: Option<impl plait::PartialHtml> = None::<EmptyHtml>,
    ) {
        article {
            h2 { (title.as_ref()) }
            (body)
            if let Some(content) = footer { footer { (content) } }
        }
    }
}

#[test]
fn anonymous_defaults_select_independent_types() {
    assert_eq!(
        html! { @ArticleCard() {} }.to_html(),
        "<article><h2>Untitled</h2><p>No content yet</p></article>"
    );
    assert_eq!(
        html! { @ArticleCard(title: String::from("Draft")) {} }.to_html(),
        "<article><h2>Draft</h2><p>No content yet</p></article>"
    );
    assert_eq!(
        html! {
            @ArticleCard(
                title: "Release notes",
                body: html! { p { "Defaults are here" } },
                footer: Some(html! { small { "Published today" } }),
            ) {}
        }
        .to_html(),
        "<article><h2>Release notes</h2><p>Defaults are here</p><footer><small>Published today</small></footer></article>"
    );
    assert_eq!(
        html! {
            @ArticleCard(footer: None::<EmptyHtml>) {}
        }
        .to_html(),
        "<article><h2>Untitled</h2><p>No content yet</p></article>"
    );
    assert_eq!(
        html! { @ArticleCard(body: EmptyHtml, footer: Some(EmptyHtml)) {} }.to_html(),
        "<article><h2>Untitled</h2><footer></footer></article>"
    );
}

component! {
    pub fn MetricCard<'a, T: Default + RenderEscaped, const N: usize>(
        current: T,
        previous: T = T::default(),
        label: &'a str = "Requests",
        history: [u8; N] = [0; N],
        annotation: &impl RenderEscaped = &"No change",
        thresholds: &[u8] = &[1, 2],
        unit: Option<&str> = None,
    ) {
        section(class: "metric", data_samples: history.len(), data_thresholds: thresholds.len()) {
            h2 { (label) }
            p {
                (current) " / " (previous)
                if let Some(unit) = unit { " " (unit) }
            }
            aside { (annotation) }
        }
    }
}

#[test]
fn defaults_preserve_generics_borrows_and_coercions() {
    assert_eq!(
        html! {
            @MetricCard::<u8, 3>(current: 7) {}
        }
        .to_html(),
        "<section class=\"metric\" data-samples=\"3\" data-thresholds=\"2\"><h2>Requests</h2><p>7 / 0</p><aside>No change</aside></section>"
    );

    let owner = String::from("Latency");
    let label = owner.as_str();
    assert_eq!(
        html! {
            @MetricCard::<&str, 0>(current: label) {}
        }
        .to_html(),
        "<section class=\"metric\" data-samples=\"0\" data-thresholds=\"2\"><h2>Requests</h2><p>Latency / </p><aside>No change</aside></section>"
    );

    let numbers = [3, 4, 5, 6];
    let thresholds = &numbers;
    let fragment = html! { strong { "Improving" } };
    let annotation = &fragment;
    let page = html! {
        @MetricCard(current: 1u16, history: [0; 2], label, annotation, thresholds, unit: None) {}
        @MetricCard::<String, 0>(current: String::from("Healthy"), previous: String::from("Degraded")) {}
    };
    for _ in 0..2 {
        assert_eq!(
            page.to_html(),
            concat!(
                "<section class=\"metric\" data-samples=\"2\" data-thresholds=\"4\"><h2>Latency</h2><p>1 / 0</p><aside><strong>Improving</strong></aside></section>",
                "<section class=\"metric\" data-samples=\"0\" data-thresholds=\"2\"><h2>Requests</h2><p>Healthy / Degraded</p><aside>No change</aside></section>",
            )
        );
    }
}

fn __plait_default_0() -> &'static str {
    "scope"
}

type P1 = bool;

component! {
    fn HygienicDefaults<'plait_0, r#P0: RenderEscaped, __PlaitProps: RenderEscaped, __PlaitState0: RenderEscaped>(
        __plait_props: &str = __plait_default_0(),
        __plait_resolve: impl AsRef<str> = "resolver",
        __plait_component: &'plait_0 str = "writer",
        r#type: r#P0,
        flag: P1 = true,
        attrs: __PlaitProps,
        children: __PlaitState0,
    ) {
        div(#attrs) {
            (__plait_props) (__plait_resolve.as_ref()) (__plait_component)
            if *flag { (r#type) (attrs) (children) #children }
        }
    }
}

#[test]
fn generated_names_do_not_capture_user_names() {
    let r#type = "type";
    assert_eq!(html! {
        @HygienicDefaults(r#type, attrs: "attrs", children: "children"; id: "id") { "forwarded" }
    }.to_html(), "<div id=\"id\">scoperesolverwritertypeattrschildrenforwarded</div>");
}
