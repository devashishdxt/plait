# plait-macros

Procedural macros for the Plait HTML templating library.

This crate provides the `html!` and `component!` macros that enable type-safe, compile-time HTML generation
with a macro-based syntax.

## Overview

- `html!` - Generate HTML fragments with embedded Rust expressions
- `component!` - Define reusable HTML components with props and children

## Quick start

```rust
use plait::{html, component, render};

// Simple HTML generation
let page = render(html! {
    div(class: "container") {
        h1 { "Hello, World!" }
        p { "Welcome to Plait." }
    }
});

// Define a reusable component
component! {
    pub fn Card<'a>(title: &'a str) {
        div(class: "card", #attrs) {
            h2 { (title) }
            #children
        }
    }
}

// Use the component
let card = render(html! {
    @Card(title: "My Card"; id: "card-1") {
        p { "Card content goes here." }
    }
});
```

## Features

- **Type-safe**: Compile-time validation of HTML structure and Rust expressions
- **XSS protection**: Automatic HTML escaping with opt-out for trusted content
- **URL validation**: Dangerous protocols in URL attributes are automatically stripped
- **Ergonomic syntax**: `snake_case` to `kebab-case` conversion for element and attribute names
- **Full Rust integration**: Conditionals, loops, and pattern matching within templates
- **Component system**: Reusable components with props, children, and attribute spreading

## Crate organization

This is a proc-macro crate and should typically be used through the main `plait` crate, which re-exports these
macros along with the runtime types (`HtmlFormatter`, `render`, etc.).

See the individual macro documentation for complete syntax references and examples.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))
- MIT license ([LICENSE-MIT](../LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
