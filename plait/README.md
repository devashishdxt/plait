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

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
