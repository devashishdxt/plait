# plait

A modern, type-safe HTML templating library for Rust that embraces composition.

Plait lets you write HTML directly in Rust using the `html!` macro, with compile-time validation, automatic
escaping, and a natural syntax that mirrors standard HTML and Rust control flow. Reusable components are defined
with the `component!` macro.

## Quick start

```rust
use plait::{html, ToHtml};

let name = "World";
let page = html! {
    div(class: "greeting") {
        h1 { "Hello, " (name) "!" }
    }
};

assert_eq!(page.to_html(), r#"<div class="greeting"><h1>Hello, World!</h1></div>"#);
```

The `html!` macro returns an `HtmlFragment` that implements `ToHtml`. Call `.to_html()`(ToHtml::to_html) to
get an `Html` value (a `String` wrapper that implements `Display`(std::fmt::Display)).

## Syntax reference

### Elements

Write element names directly. Children go inside braces. Void elements (like `br`, `img`, `input`) use a semicolon
instead.

```rust
let frag = html! {
    div {
        p { "Hello" }
        br;
        img(src: "/logo.png");
    }
};
```

Snake-case identifiers are automatically converted to kebab-case:

```rust
// Renders as <my-element>...</my-element>
let frag = html! { my_element { "content" } };
assert_eq!(frag.to_html(), "<my-element>content</my-element>");
```

### DOCTYPE

Use `#doctype` to emit `<!DOCTYPE html>`:

```rust
let page = html! {
    #doctype
    html {
        head { title { "My Page" } }
        body { "Hello" }
    }
};

assert_eq!(page.to_html(), "<!DOCTYPE html><html><head><title>My Page</title></head><body>Hello</body></html>");
```

### Text and expressions

String literals are rendered as static text (HTML-escaped). Rust expressions inside parentheses are also
HTML-escaped by default. Use `#(expr)` for raw (unescaped) output.

```rust
let user = "<script>alert('xss')</script>";
let frag = html! {
    "Static text "
    (user)              // escaped: &lt;script&gt;...
    #("<b>bold</b>")    // raw: <b>bold</b>
};
```

Expressions in `()` must implement `RenderEscaped`. Expressions in `#()` must implement `RenderRaw`.

### Attributes

Attributes go in parentheses after the element name.

```rust
let frag = html! {
    // String value
    div(class: "container", id: "main") { "content" }

    // Boolean attribute (no value) - always rendered
    button(disabled) { "Can't click" }

    // Expression value (escaped)
    input(type: "text", value: ("hello"));

    // Raw expression value (unescaped)
    div(class: #("raw-class")) {}
};
```

Underscore-to-hyphen conversion applies to attribute names too:

```rust
// Renders as hx-target="body"
let frag = html! { div(hx_target: "body") {} };

assert_eq!(frag.to_html(), "<div hx-target=\"body\"></div>");
```

Use string literals for attribute names that need special characters:

```rust
let frag = html! { div("@click": "handler()") {} };

assert_eq!(frag.to_html(), r#"<div @click="handler()"></div>"#);
```

### Optional attributes

Append `?` to the attribute name (before the `:`) to make it conditional. The attribute is only rendered when the
value is `Some(_)` (for `Option`) or `true` (for `bool`).

```rust
let class = Some("active");
let disabled = false;

let frag = html! {
    button(class?: class, disabled?: disabled) { "Click" }
};
assert_eq!(frag.to_html(), r#"<button class="active">Click</button>"#);
```

Values for `?` attributes must implement `RenderMaybeAttributeEscaped` (or `RenderMaybeAttributeRaw` when used
with `#()`).

### Control flow

Standard Rust `if`/`else`, `if let`, `for`, and `match` work inside templates:

```rust
let items = vec!["one", "two", "three"];
let show_header = true;

let frag = html! {
    if show_header {
        h1 { "List" }
    }

    ul {
        for item in items.iter() {
            li { (item) }
        }
    }
};

```

```rust
let value = Some("hello");

let frag = html! {
    if let Some(v) = value {
        span { (v) }
    } else {
        span { "nothing" }
    }
};

```

```rust
let tag = "div";

let frag = html! {
    match tag {
        "div" => div { "a div" },
        "span" => span { "a span" },
        _ => "unknown"
    }
};

```

### Let bindings

Compute intermediate values within templates:

```rust
let world = "World";

let frag = html! {
    let len = world.len();
    "Length: " (len)
};
assert_eq!(frag.to_html(), "Length: 5");
```

### Nesting fragments

`HtmlFragment` implements `RenderEscaped`, so fragments can be embedded in other fragments:

```rust
let inner = html! { p { "inner content" } };
let outer = html! { div { (inner) } };
assert_eq!(outer.to_html(), "<div><p>inner content</p></div>");
```

## Components

Components are reusable template functions defined with the `component!` macro:

```rust
use plait::{component, classes, Class};

component! {
    pub fn Button(class: impl Class) {
        button(class: classes!("btn", class), #attrs) {
            #children
        }
    }
}
```

The macro generates a struct and a `Component` trait implementation. Components are
called with `@` syntax inside `html!`:

```rust
let page = html! {
    @Button(class: "primary"; id: "submit-btn", disabled?: false) {
        "Submit"
    }
};

assert_eq!(
    page.to_html(),
    r#"<button class="btn primary" id="submit-btn">Submit</button>"#
);
```

In the component call, props appear before the `;`, and extra HTML attributes appear after. The component body uses
`#attrs` to spread those extra attributes and `#children` to render the child content.

### Shorthand props

When a variable has the same name as a component prop, you can use shorthand syntax - just like Rust struct
initialization:

```rust
let class = "primary";

// These are equivalent:
let a = html! { @Button(class: class) { "Click" } };
let b = html! { @Button(class) { "Click" } };

assert_eq!(a.to_html(), b.to_html());
```

Shorthand and explicit props can be mixed freely:

```rust
let name = "Alice";
let html = html! { @UserCard(name, role: "Admin") {} };

assert_eq!(html.to_html(), "<div><span>Alice</span> - <span>Admin</span></div>");
```

### Passing fragments as props

Use `PartialHtml` as a prop bound to accept `html!` output as a component prop:

```rust
component! {
    pub fn Card(title: impl PartialHtml) {
        div(class: "card") {
            h1 { (title) }
            #children
        }
    }
}

let page = html! {
    @Card(title: html! { span { "My Card" } }) {
        p { "Card body" }
    }
};
```

### Primitive props

Component props are received as references. For primitive types like `bool` or `u32`, dereference with `*` in the
component body:

```rust
component! {
    pub fn Badge(count: u32, visible: bool) {
        if *visible {
            span(class: "badge") { (count) }
        }
    }
}
```

## CSS classes

The `classes!` macro combines multiple class values, automatically skipping empty strings and `None` values:

```rust
let extra: Option<&str> = None;

let frag = html! {
    div(class: classes!("base", "primary", extra)) {}
};
assert_eq!(frag.to_html(), r#"<div class="base primary"></div>"#);
```

Values passed to `classes!` must implement the `Class` trait. This is implemented for `&str`, `Option<T>` where
`T: Class`, and `Classes<T>`(Classes).

## Web framework integrations

Plait provides optional integrations with popular Rust web frameworks. Both `Html` and `HtmlFragment` can be
returned directly from request handlers when the corresponding feature is enabled.

Enable integrations by adding the feature flag to your `Cargo.toml`:

```toml
[dependencies]
plait = { version = "0.8", features = ["axum"] }
```

Available features: `actix-web`, `axum`, `rocket`.

### axum

`Html` and `HtmlFragment` implement
`IntoResponse`(https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html):

```rust
use axum::{Router, routing::get};
use plait::{html, ToHtml};

async fn index() -> plait::Html {
    html! {
        h1 { "Hello from plait!" }
    }.to_html()
}

let app = Router::new().route("/", get(index));
```

You can also return an `HtmlFragment` directly without calling `.to_html()`:

```rust
async fn index() -> impl axum::response::IntoResponse {
    plait::html! {
        h1 { "Hello from plait!" }
    }
}
```

### actix-web

`Html` and `HtmlFragment` implement
`Responder`(https://docs.rs/actix-web/latest/actix_web/trait.Responder.html):

```rust
use actix_web::{App, HttpServer, get};
use plait::{html, ToHtml};

#[get("/")]
async fn index() -> plait::Html {
    html! {
        h1 { "Hello from plait!" }
    }.to_html()
}
```

### rocket

`Html` and `HtmlFragment` implement
`Responder`(https://docs.rs/rocket/latest/rocket/response/trait.Responder.html):

```rust
use rocket::get;
use plait::{html, ToHtml};

#[get("/")]
fn index() -> plait::Html {
    html! {
        h1 { "Hello from plait!" }
    }.to_html()
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
