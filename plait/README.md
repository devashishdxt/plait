# plait

A fast, type-safe HTML templating library for Rust.

Plait provides compile-time HTML generation with a concise, Rust-native syntax. Templates are checked at compile
time and generate efficient code with minimal runtime overhead.

## Quick Start

```rust
use plait::{html, render};

let name = "World";
let template = html! {
    div class="greeting" {
        h1 { "Hello, " (name) "!" }
    }
};

let html = render(template);
assert_eq!(html, r#"<div class="greeting"><h1>Hello, World!</h1></div>"#);
```

## Macros

Plait provides two macros:

- `html!` - Creates a `RenderFn` from a template, which can be rendered using `render()`
- `attrs!` - Creates an `Attributes` collection for use in templates

## Template Syntax

### Elements

Elements are written as `name { children }` for normal elements or `name;` for void elements:

```rust
use plait::{html, render};

let template = html! {
    div {
        p { "A paragraph" }
        br;
        input type="text" name="field";
    }
};

assert_eq!(render(template), r#"<div><p>A paragraph</p><br><input type="text" name="field"></div>"#);
```

### Attributes

Attributes support several value types:

```rust
use plait::{html, render};

let class_name = "container";
let maybe_id: Option<&str> = Some("main");
let is_disabled = true;

let template = html! {
    div
        class="literal"              // Literal string
        data-value=(class_name)      // Dynamic expression
        id=[maybe_id]                // Optional (renders if Some)
        disabled?[is_disabled]       // Boolean (renders if true)
    {
        "content"
    }
};

assert_eq!(render(template), r#"<div class="literal" data-value="container" id="main" disabled>content</div>"#);
```

### Dynamic Content

Expressions in parentheses are escaped by default:

```rust
use plait::{html, render};

let user_input = "<script>alert('xss')</script>";

let template = html! {
    div { (user_input) }
};

// Content is safely escaped
assert!(!render(template).contains("<script>"));
```

Use `: raw` to include pre-escaped content:

```rust
use plait::{html, render};

let trusted_html = "<strong>Bold</strong>";

let template = html! {
    div { (trusted_html : raw) }
};

assert!(render(template).contains("<strong>"));
```

### Control Flow

#### Conditionals

```rust
use plait::{html, render};

let show = true;
let value: Option<&str> = Some("hello");

let template = html! {
    div {
        @if show {
            span { "Visible" }
        }

        @if let Some(v) = value {
            span { (v) }
        } @else {
            span { "No value" }
        }
    }
};

assert_eq!(render(template), r#"<div><span>Visible</span><span>hello</span></div>"#);
```

#### Loops

```rust
use plait::{html, render};

let items = vec!["one", "two", "three"];

let template = html! {
    ul {
        @for item in &items {
            li { (item) }
        }
    }
};

assert_eq!(render(template), r#"<ul><li>one</li><li>two</li><li>three</li></ul>"#);
```

#### Match Expressions

```rust
use plait::{html, render};

enum Status { Active, Inactive }
let status = Status::Active;

let template = html! {
    span {
        @match status {
            Status::Active => "Online",
            Status::Inactive => "Offline",
        }
    }
};

assert_eq!(render(template), r#"<span>Online</span>"#);
```

## Custom Components

Create reusable components as functions that return `impl Render`:

```rust
use plait::{Render, html, render};

fn button(label: &str, primary: bool) -> impl Render {
    let class = if primary { "btn btn-primary" } else { "btn" };
    html! {
        button class=(class) { (label) }
    }
}

// Use in templates
let template = html! {
    div { (button("Click me", true)) }
};

assert_eq!(render(template), r#"<div><button class="btn btn-primary">Click me</button></div>"#);
```

## Safety

Plait automatically escapes dynamic content to prevent XSS vulnerabilities. The `Html` and `PreEscaped` types
represent content that is already safe and will not be escaped again.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
