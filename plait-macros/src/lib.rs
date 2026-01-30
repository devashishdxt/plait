//! Procedural macros for the Plait HTML templating library.
//!
//! This crate provides the [`html!`] and [`component!`] macros that enable type-safe, compile-time HTML generation
//! with a macro-based syntax.
//!
//! # Overview
//!
//! - [`html!`] - Generate HTML fragments with embedded Rust expressions
//! - [`component!`] - Define reusable HTML components with props and children
//!
//! # Quick start
//!
//! ```rust,ignore
//! use plait::{html, component, render};
//!
//! // Simple HTML generation
//! let page = render(html! {
//!     div(class: "container") {
//!         h1 { "Hello, World!" }
//!         p { "Welcome to Plait." }
//!     }
//! });
//!
//! // Define a reusable component
//! component! {
//!     pub fn Card<'a>(title: &'a str) {
//!         div(class: "card", #attrs) {
//!             h2 { (title) }
//!             #children
//!         }
//!     }
//! }
//!
//! // Use the component
//! let card = render(html! {
//!     @Card(title: "My Card"; id: "card-1") {
//!         p { "Card content goes here." }
//!     }
//! });
//! ```
//!
//! # Features
//!
//! - **Type-safe**: Compile-time validation of HTML structure and Rust expressions
//! - **XSS protection**: Automatic HTML escaping with opt-out for trusted content
//! - **URL validation**: Dangerous protocols in URL attributes are automatically stripped
//! - **Ergonomic syntax**: `snake_case` to `kebab-case` conversion for element and attribute names
//! - **Full Rust integration**: Conditionals, loops, and pattern matching within templates
//! - **Component system**: Reusable components with props, children, and attribute spreading
//!
//! # Crate organization
//!
//! This is a proc-macro crate and should typically be used through the main `plait` crate, which re-exports these
//! macros along with the runtime types (`HtmlFormatter`, `render`, etc.).
//!
//! See the individual macro documentation for complete syntax references and examples.

mod ast;
mod codegen;

use proc_macro::TokenStream;

/// A procedural macro for generating type-safe HTML with embedded Rust expressions.
///
/// The `html!` macro provides a macro-based syntax for creating HTML content at compile time. It returns a closure
/// that takes `&mut HtmlFormatter<'_>` and writes the HTML output.
///
/// # Basic Usage
///
/// ```rust,ignore
/// use plait::{html, render};
///
/// let html = render(html! {
///     div {
///         "Hello World"
///     }
/// });
/// // Output: <div>Hello World</div>
/// ```
///
/// # Syntax reference
///
/// ## Text content
///
/// String literals are automatically HTML-escaped:
///
/// ```rust,ignore
/// html! { "<script>alert('XSS')</script>" }
/// // Output: &lt;script&gt;alert('XSS')&lt;/script&gt;
/// ```
///
/// ## Expressions
///
/// Rust expressions can be embedded using parentheses. Values are HTML-escaped:
///
/// ```rust,ignore
/// let name = "World";
/// html! { "Hello " (name) }
/// // Output: Hello World
/// ```
///
/// ## Raw (unescaped) output
///
/// Use `#(expr)` to output content without HTML escaping:
///
/// ```rust,ignore
/// html! { #("<strong>Bold</strong>") }
/// // Output: <strong>Bold</strong>
/// ```
///
/// **Warning:** Only use raw output with trusted content to prevent XSS vulnerabilities.
///
/// ## Elements
///
/// HTML elements use a block syntax with the tag name followed by braces:
///
/// ```rust,ignore
/// html! {
///     div {
///         span { "Nested content" }
///     }
/// }
/// // Output: <div><span>Nested content</span></div>
/// ```
///
/// ### Element name conversion
///
/// Element names are automatically converted from `snake_case` to `kebab-case`:
///
/// ```rust,ignore
/// html! { custom_element { "Content" } }
/// // Output: <custom-element>Content</custom-element>
/// ```
///
/// ### Void elements
///
/// Void elements (`br`, `hr`, `img`, `input`, `meta`, `link`, etc.) use a semicolon
/// instead of braces:
///
/// ```rust,ignore
/// html! {
///     div {
///         br;
///         input(type: "text");
///     }
/// }
/// // Output: <div><br><input type="text"></div>
/// ```
///
/// ## Attributes
///
/// Attributes are specified in parentheses after the element name:
///
/// ```rust,ignore
/// html! {
///     div(class: "container", id: "main") {
///         "Content"
///     }
/// }
/// // Output: <div class="container" id="main">Content</div>
/// ```
///
/// ### Attribute name conversion
///
/// Attribute names with underscores are converted to hyphens:
///
/// ```rust,ignore
/// html! { div(hx_target: "body") {} }
/// // Output: <div hx-target="body"></div>
/// ```
///
/// ### String literal attribute names
///
/// Use string literals for attribute names with special characters:
///
/// ```rust,ignore
/// html! { div("@click": "handleClick()") {} }
/// // Output: <div @click="handleClick()"></div>
/// ```
///
/// ### Boolean attributes
///
/// Attributes without values render as boolean attributes:
///
/// ```rust,ignore
/// html! { button(disabled) { "Submit" } }
/// // Output: <button disabled>Submit</button>
/// ```
///
/// ### Optional attributes
///
/// Use `?:` syntax for conditional attributes. Works with `Option<T>` and `bool`:
///
/// ```rust,ignore
/// let class = Some("active");
/// let disabled = false;
///
/// html! {
///     button(class?: class, disabled?: disabled) {
///         "Click"
///     }
/// }
/// // Output: <button class="active">Click</button>
/// ```
///
/// ### Raw attribute values
///
/// Use `#(expr)` for unescaped attribute values:
///
/// ```rust,ignore
/// html! { div(class: #("<script>")) {} }
/// // Output: <div class="<script>"></div>
/// ```
///
/// ### URL attribute protection
///
/// URL attributes (`href`, `src`, `action`, etc.) are automatically validated.
/// Dangerous protocols like `javascript:` are stripped:
///
/// ```rust,ignore
/// html! { a(href: "javascript:alert('XSS')") { "Link" } }
/// // Output: <a>Link</a>
///
/// // Safe URLs work normally:
/// html! { a(href: "https://example.com") { "Link" } }
/// // Output: <a href="https://example.com">Link</a>
/// ```
///
/// Use `#(expr)` to bypass URL validation when needed (trusted content only).
///
/// ## Control flow
///
/// ### If expressions
///
/// ```rust,ignore
/// let show = true;
///
/// html! {
///     if show {
///         div { "Visible" }
///     }
/// }
/// ```
///
/// With `else` and `else if`:
///
/// ```rust,ignore
/// let count = 5;
///
/// html! {
///     if count == 0 {
///         "None"
///     } else if count == 1 {
///         "One"
///     } else {
///         "Many"
///     }
/// }
/// ```
///
/// ### If let expressions
///
/// ```rust,ignore
/// let user = Some("Alice");
///
/// html! {
///     if let Some(name) = user {
///         span { "Hello, " (name) }
///     } else {
///         span { "Guest" }
///     }
/// }
/// ```
///
/// ### For loops
///
/// ```rust,ignore
/// let items = vec!["Apple", "Banana", "Cherry"];
///
/// html! {
///     ul {
///         for item in items {
///             li { (item) }
///         }
///     }
/// }
/// // Output: <ul><li>Apple</li><li>Banana</li><li>Cherry</li></ul>
/// ```
///
/// ### Match expressions
///
/// ```rust,ignore
/// let status = "success";
///
/// html! {
///     match status {
///         "success" => span(class: "green") { "OK" },
///         "error" => span(class: "red") { "Failed" },
///         _ => span { "Unknown" }
///     }
/// }
/// ```
///
/// ## Component calls
///
/// Components are invoked with `@ComponentName` syntax. See the [`component!`] macro for defining components.
///
/// ```rust,ignore
/// html! {
///     @Button(class: "primary"; id: "submit", disabled?: false) {
///         "Click me"
///     }
/// }
/// ```
///
/// Component calls use a semicolon to separate component props from HTML attributes:
/// - Before `;`: Component-specific props
/// - After `;`: HTML attributes passed through `#attrs`
///
/// # Generated code
///
/// The macro generates a closure of type `impl FnOnce(&mut HtmlFormatter<'_>)`. Use with `render()` or
/// `render_with_capacity()` to produce the final HTML string.
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    codegen::html_impl(input.into()).into()
}

/// A procedural macro for defining reusable HTML components.
///
/// The `component!` macro creates a struct and implements the `Component` trait, enabling composable and reusable
/// HTML templates.
///
/// # Basic usage
///
/// ```rust,ignore
/// use plait::{component, html, render};
///
/// component! {
///     pub fn Card {
///         div(class: "card") {
///             #children
///         }
///     }
/// }
///
/// let html = render(html! {
///     @Card {
///         "Card content"
///     }
/// });
/// // Output: <div class="card">Card content</div>
/// ```
///
/// # Syntax
///
/// ```rust,ignore
/// component! {
///     [visibility] fn ComponentName[<generics>]([props]) [where clause] {
///         // component body (html! syntax)
///     }
/// }
/// ```
///
/// ## Component props
///
/// Define typed props in the function signature:
///
/// ```rust,ignore
/// component! {
///     pub fn Button<'a>(class: &'a str, size: u32) {
///         button(class: format_args!("btn {} size-{}", class, size)) {
///             #children
///         }
///     }
/// }
///
/// // Usage:
/// html! {
///     @Button(class: "primary", size: 2) {
///         "Click me"
///     }
/// }
/// // Output: <button class="btn primary size-2">Click me</button>
/// ```
///
/// ## Special placeholders
///
/// ### `#children` - Child content
///
/// The `#children` placeholder renders content passed between the component's opening and closing tags:
///
/// ```rust,ignore
/// component! {
///     pub fn Wrapper {
///         div(class: "wrapper") {
///             #children
///         }
///     }
/// }
///
/// html! {
///     @Wrapper {
///         span { "Child 1" }
///         span { "Child 2" }
///     }
/// }
/// // Output: <div class="wrapper"><span>Child 1</span><span>Child 2</span></div>
/// ```
///
/// ### `#attrs` - Attribute spreading
///
/// The `#attrs` placeholder spreads HTML attributes passed to the component. This enables attribute forwarding for
/// flexible component APIs:
///
/// ```rust,ignore
/// component! {
///     pub fn Button<'a>(class: &'a str) {
///         button(class: format_args!("btn {}", class), #attrs) {
///             #children
///         }
///     }
/// }
///
/// // Attributes after `;` are spread via #attrs:
/// html! {
///     @Button(class: "primary"; id: "submit", disabled?: true) {
///         "Submit"
///     }
/// }
/// // Output: <button class="btn primary" id="submit" disabled>Submit</button>
/// ```
///
/// ## Generics and lifetimes
///
/// Components support full Rust generics:
///
/// ```rust,ignore
/// component! {
///     pub fn List<'a, T: std::fmt::Display>(items: &'a [T]) {
///         ul {
///             for item in items {
///                 li { (item) }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Component composition
///
/// Components can be nested and composed:
///
/// ```rust,ignore
/// component! {
///     pub fn Button<'a>(class: &'a str) {
///         button(class: format_args!("btn {}", class), #attrs) {
///             #children
///         }
///     }
/// }
///
/// component! {
///     pub fn Card {
///         div(class: "card") {
///             @Button(class: "card-btn"; #attrs) {
///                 #children
///             }
///         }
///     }
/// }
/// ```
///
/// # Calling components
///
/// Components are called using `@ComponentName` syntax within `html!`:
///
/// ```rust,ignore
/// html! {
///     @ComponentName(prop1: value1, prop2: value2; attr1: val1, attr2?: optional) {
///         // children
///     }
/// }
/// ```
///
/// The syntax is:
/// - `@ComponentName` - Component invocation prefix
/// - `(...)` - Optional arguments section
///   - Before `;` - Component props (matched to component parameters)
///   - After `;` - HTML attributes (spread via `#attrs`)
/// - `{ ... }` - Children content (rendered via `#children`)
///
/// If a component has no props, you can omit everything before the semicolon:
///
/// ```rust,ignore
/// html! {
///     @Card(; class: "highlighted") {
///         "Content"
///     }
/// }
/// ```
///
/// # Generated code
///
/// The macro generates:
///
/// 1. A struct with the component's props as public fields:
///    ```rust,ignore
///    pub struct Button<'a> {
///        pub class: &'a str,
///    }
///    ```
///
/// 2. An implementation of the `Component` trait:
///    ```rust,ignore
///    impl<'a> Component for Button<'a> {
///        fn render(
///            self,
///            f: &mut HtmlFormatter<'_>,
///            attrs: impl FnOnce(&mut HtmlFormatter<'_>),
///            children: impl FnOnce(&mut HtmlFormatter<'_>),
///        ) {
///            // component body
///        }
///    }
///    ```
#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    codegen::component_impl(input.into()).into()
}
