//! A lightweight, type-safe HTML templating library for Rust.
//!
//! Plait provides a macro-based DSL for writing HTML directly in Rust code with compile-time validation and automatic
//! escaping. It's designed for building server-side rendered HTML with minimal runtime overhead.
//!
//! # Quick Start
//!
//! The [`html!`] macro returns a value that implements [`Display`](core::fmt::Display), so you can render it with
//! `.to_string()`, `write!`, or `format!`.
//!
//! ```rust
//! use plait::html;
//!
//! let name = "World";
//! let page = html! {
//!     div(class: "greeting") {
//!         h1 { "Hello, " (name) "!" }
//!     }
//! };
//!
//! assert_eq!(page.to_string(), "<div class=\"greeting\"><h1>Hello, World!</h1></div>");
//! ```
//!
//! # The `html!` Macro
//!
//! The [`html!`] macro provides a concise syntax for writing HTML:
//!
//! ```rust
//! use plait::html;
//!
//! let page = html! {
//!     // Elements with attributes
//!     div(id: "main", class: "container") {
//!         // Nested elements
//!         h1 { "Title" }
//!
//!         // Self-closing void elements
//!         br;
//!         input(type: "text", name: "query");
//!
//!         // Text content and expressions (escaped by default)
//!         p { "Static text and " (2 + 2) " dynamic values" }
//!     }
//! };
//!
//! assert_eq!(
//!     page.to_string(),
//!     "<div id=\"main\" class=\"container\">\
//!      <h1>Title</h1>\
//!      <br>\
//!      <input type=\"text\" name=\"query\">\
//!      <p>Static text and 4 dynamic values</p>\
//!      </div>",
//! );
//! ```
//!
//! ## DOCTYPE
//!
//! Use `#doctype` to emit `<!DOCTYPE html>`:
//!
//! ```rust
//! use plait::html;
//!
//! let page = html! {
//!     #doctype
//!     html {
//!         head { title { "My Page" } }
//!         body { "Hello" }
//!     }
//! };
//!
//! assert!(page.to_string().starts_with("<!DOCTYPE html>"));
//! ```
//!
//! ## Expressions
//!
//! Use parentheses to embed Rust expressions. Content is automatically HTML-escaped:
//!
//! ```rust
//! use plait::html;
//!
//! let user_input = "<script>alert('xss')</script>";
//! let html = html! { p { (user_input) } };
//!
//! assert_eq!(html.to_string(), "<p>&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;</p>");
//! ```
//!
//! For raw (unescaped) content, prefix with `#`:
//!
//! ```rust
//! use plait::html;
//!
//! let trusted_html = "<strong>Bold</strong>";
//! let html = html! { div { #(trusted_html) } };
//!
//! assert_eq!(html.to_string(), "<div><strong>Bold</strong></div>");
//! ```
//!
//! ## Let Bindings
//!
//! Use `let` bindings to compute intermediate values:
//!
//! ```rust
//! use plait::html;
//!
//! let world = " World";
//! let html = html! {
//!     let hello = world.len();
//!     (hello) (world)
//! };
//!
//! assert_eq!(html.to_string(), "6 World");
//! ```
//!
//! ## Attributes
//!
//! Attributes support several forms:
//!
//! ```rust
//! use plait::html;
//!
//! let class = Some("active");
//! let disabled = true;
//!
//! let html = html! {
//!     // Boolean attribute (no value)
//!     button(checked) { "Checked" }
//!
//!     // Optional attribute with `?:` - included when Some or true, omitted when None or false
//!     div(class?: class) { "Has class" }
//!     button(disabled?: disabled) { "Disabled" }
//!
//!     // Attribute names with underscores are converted to hyphens
//!     div(hx_target: "body") {}
//!
//!     // String attribute names for special characters
//!     div("@click": "handler()") {}
//!
//!     // Raw (unescaped) attribute value with `#(...)`
//!     div(class: #("<raw>")) {}
//! };
//!
//! assert_eq!(
//!     html.to_string(),
//!     "<button checked>Checked</button>\
//!      <div class=\"active\">Has class</div>\
//!      <button disabled>Disabled</button>\
//!      <div hx-target=\"body\"></div>\
//!      <div @click=\"handler()\"></div>\
//!      <div class=\"<raw>\"></div>",
//! );
//! ```
//!
//! ## Control Flow
//!
//! Standard Rust control flow works naturally:
//!
//! ```rust
//! use plait::html;
//!
//! let show = true;
//! let items = vec!["a", "b", "c"];
//! let variant = "primary";
//! let user = Some("Alice");
//!
//! let html = html! {
//!     // Conditionals
//!     if show {
//!         p { "Visible" }
//!     } else {
//!         p { "Hidden" }
//!     }
//!
//!     // if let
//!     if let Some(name) = user {
//!         p { "Welcome, " (name) "!" }
//!     } else {
//!         p { "Please log in" }
//!     }
//!
//!     // Loops
//!     ul {
//!         for item in &items {
//!             li { (item) }
//!         }
//!     }
//!
//!     // Pattern matching
//!     match variant {
//!         "primary" => button(class: "btn-primary") { "Primary" },
//!         "secondary" => button(class: "btn-secondary") { "Secondary" },
//!         _ => button { "Default" }
//!     }
//! };
//!
//! assert_eq!(
//!     html.to_string(),
//!     "<p>Visible</p>\
//!      <p>Welcome, Alice!</p>\
//!      <ul><li>a</li><li>b</li><li>c</li></ul>\
//!      <button class=\"btn-primary\">Primary</button>",
//! );
//! ```
//!
//! ## Nesting HTML Fragments
//!
//! Use `@(expr)` to embed a value that implements [`HtmlDisplay`] (such as another [`html!`] fragment) without
//! escaping:
//!
//! ```rust
//! use plait::html;
//!
//! let inner = html! { p { "Hello World" } };
//! let outer = html! { div { @(&inner) } };
//!
//! assert_eq!(outer.to_string(), "<div><p>Hello World</p></div>");
//! ```
//!
//! ## Ownership and Borrowing
//!
//! [`html!`] expands into a `move` closure that implements [`Fn`] - it must be callable more than once (e.g. via
//! [`Display::fmt`](core::fmt::Display::fmt)). Values used in the template are moved into the closure, but because
//! the closure is [`Fn`], its captures live behind a shared reference and cannot be moved out.
//!
//! For [`Copy`] types like `&str`, `i32`, or `bool`, this is invisible - they are copied each time the closure runs.
//! For owned types like [`String`] or [`Vec`], you must explicitly borrow with `&` inside the template:
//!
//! ```rust,compile_fail
//! use plait::html;
//!
//! let name = String::from("World");
//!
//! // ERROR: cannot move `name` out of the `Fn` closure
//! let fragment = html! { p { (name) } };
//! ```
//!
//! Use `&` to borrow the captured value instead:
//!
//! ```rust
//! use plait::html;
//!
//! let name = String::from("World");
//!
//! let fragment = html! { p { (&name) } };
//!
//! assert_eq!(fragment.to_string(), "<p>World</p>");
//! ```
//!
//! The same applies anywhere a value is used inside the template - element children, attribute values, loop
//! iterators, etc. When in doubt, borrow with `&`.
//!
//! # Components
//!
//! Create reusable components using the [`component!`] macro:
//!
//! ```rust
//! use plait::{component, html, classes, ClassPart};
//!
//! component! {
//!     fn Button(class: impl ClassPart) {
//!         button(class: classes!("btn", class), #attrs) {
//!             #children
//!         }
//!     }
//! }
//!
//! let html = html! {
//!     // Component props before `;`, extra HTML attributes after
//!     @Button(class: "primary"; id: "submit-btn", disabled?: false) {
//!         "Click me"
//!     }
//! };
//!
//! assert_eq!(html.to_string(), "<button class=\"btn primary\" id=\"submit-btn\">Click me</button>");
//! ```
//!
//! Inside components, `#attrs` spreads additional HTML attributes passed at the call site and `#children` renders the
//! component's child content.
//!
//! ## Passing HTML as Props
//!
//! Components can accept [`html!`] fragments as props using the [`HtmlDisplay`] trait:
//!
//! ```rust
//! use plait::{HtmlDisplay, component, html};
//!
//! component! {
//!     fn Card(title: impl HtmlDisplay) {
//!         div(class: "card") {
//!             h1 { @(title) }
//!             #children
//!         }
//!     }
//! }
//!
//! let html = html! {
//!     @Card(title: html! { span(class: "highlight") { "My Title" } }) {
//!         p { "Card content" }
//!     }
//! };
//!
//! assert_eq!(
//!     html.to_string(),
//!     "<div class=\"card\"><h1><span class=\"highlight\">My Title</span></h1><p>Card content</p></div>",
//! );
//! ```
//!
//! ## Component Syntax
//!
//! Components support generics, lifetimes, anonymous lifetimes, `impl Trait` parameters, and where clauses:
//!
//! ```rust
//! use plait::{HtmlDisplay, ClassPart, component, html, classes};
//!
//! // Anonymous lifetimes: `&str` is automatically desugared
//! component! {
//!     fn NavLink(href: &str, label: &str, class: impl ClassPart, active: bool) {
//!         a(href: href, class: classes!(
//!             "nav-link", class,
//!             if *active { "active" } else { "" },
//!         )) {
//!             (label)
//!         }
//!     }
//! }
//!
//! // Explicit generics with where clauses
//! component! {
//!     fn Card<H, F>(header: H, footer: F) where H: HtmlDisplay, F: HtmlDisplay {
//!         div(class: "card") {
//!             div(class: "header") { @(header) }
//!             div(class: "body") { #children }
//!             div(class: "footer") { @(footer) }
//!         }
//!     }
//! }
//! ```
//!
//! ## Prop Access
//!
//! Props are received as references inside the component body. Primitive types like `bool` and `u32` should be
//! dereferenced with `*`:
//!
//! ```rust
//! use plait::{component, html};
//!
//! component! {
//!     fn Badge(count: u32, visible: bool) {
//!         if *visible {
//!             span(class: "badge") { (count) }
//!         }
//!     }
//! }
//!
//! let html = html! {
//!     @Badge(count: 5, visible: true) {}
//! };
//!
//! assert_eq!(html.to_string(), "<span class=\"badge\">5</span>");
//! ```
//!
//! # URL Safety
//!
//! URL attributes (`href`, `src`, `action`, etc.) are automatically validated. Dangerous schemes like `javascript:`
//! are stripped:
//!
//! ```rust
//! use plait::html;
//!
//! let html = html! {
//!     a(href: "javascript:alert('xss')") { "Click" }
//! };
//!
//! assert_eq!(html.to_string(), "<a>Click</a>");  // href removed
//! ```
//!
//! Safe schemes (`http`, `https`, `mailto`, `tel`) and relative paths are allowed. Use `#(...)` for raw URLs when
//! you trust the source.
//!
//! # Merging CSS Classes
//!
//! Use [`classes!`] to combine multiple class values into a single space-separated string. Any type implementing
//! [`ClassPart`] can be used - empty strings and `None` values are automatically skipped:
//!
//! ```rust
//! use plait::{component, html, classes, ClassPart};
//!
//! component! {
//!     fn Button(class: impl ClassPart) {
//!         button(class: classes!("btn", class), #attrs) {
//!             #children
//!         }
//!     }
//! }
//!
//! let html = html! {
//!     @Button(class: Some("btn-primary")) { "Click me" }
//! };
//!
//! assert_eq!(html.to_string(), "<button class=\"btn btn-primary\">Click me</button>");
//! ```

mod classes;
mod component;
#[doc(hidden)]
pub mod display;
mod fragment;
mod maybe_attr;
mod url;

/// Produces an [`HtmlFragment`] from a concise markup DSL.
///
/// The returned value implements both [`Display`](core::fmt::Display) and [`HtmlDisplay`], so you can render it with
/// `.to_string()` / `write!` or embed it inside other templates with `@(expr)`.
///
/// # Elements
///
/// Write element names followed by braces for children, or a semicolon for void elements:
///
/// ```rust
/// use plait::html;
///
/// let html = html! {
///     div(id: "main", class: "container") {
///         h1 { "Title" }
///         br;
///         input(type: "text", name: "query");
///         p { "Static text and " (2 + 2) " dynamic values" }
///     }
/// };
///
/// assert_eq!(
///     html.to_string(),
///     "<div id=\"main\" class=\"container\">\
///      <h1>Title</h1>\
///      <br>\
///      <input type=\"text\" name=\"query\">\
///      <p>Static text and 4 dynamic values</p>\
///      </div>",
/// );
/// ```
///
/// Underscores in element names are converted to hyphens (e.g. `custom_element` becomes `<custom-element>`).
///
/// # DOCTYPE
///
/// Use `#doctype` to emit `<!DOCTYPE html>`:
///
/// ```rust
/// use plait::html;
///
/// let page = html! {
///     #doctype
///     html {
///         head { title { "My Page" } }
///         body { "Hello" }
///     }
/// };
///
/// assert!(page.to_string().starts_with("<!DOCTYPE html>"));
/// ```
///
/// # Expressions
///
/// Parenthesised expressions are HTML-escaped. Prefix with `#` for raw (unescaped) output:
///
/// ```rust
/// use plait::html;
///
/// let user = "<script>xss</script>";
/// let trusted = "<em>safe</em>";
///
/// let html = html! { p { (user) " " #(trusted) } };
///
/// assert_eq!(
///     html.to_string(),
///     "<p>&lt;script&gt;xss&lt;/script&gt; <em>safe</em></p>",
/// );
/// ```
///
/// # Let Bindings
///
/// Compute intermediate values inside the template:
///
/// ```rust
/// use plait::html;
///
/// let world = " World";
/// let html = html! {
///     let len = world.len();
///     (len) (world)
/// };
///
/// assert_eq!(html.to_string(), "6 World");
/// ```
///
/// # Attributes
///
/// | Syntax           | Behaviour                                                                        |
/// |------------------|----------------------------------------------------------------------------------|
/// | `name: expr`     | Always rendered, value is HTML-escaped.                                          |
/// | `name: #(expr)`  | Always rendered, value is **not** escaped.                                       |
/// | `name`           | Boolean attribute with no value.                                                 |
/// | `name?: expr`    | Optional - rendered when `bool` is `true` or `Option` is `Some`.                 |
/// | `name?: #(expr)` | Optional + raw.                                                                  |
/// | `"string": expr` | String key - use for names that aren't valid Rust identifiers (e.g. `"@click"`). |
///
/// Identifiers with underscores are converted to hyphens (`hx_target` → `hx-target`).
///
/// URL attributes (`href`, `src`, `action`, etc.) are validated - dangerous schemes like `javascript:` are
/// silently stripped. Use `#(...)` for raw URLs when the source is trusted.
///
/// # Control Flow
///
/// `if` / `else if` / `else`, `if let`, `for .. in`, and `match` work as in Rust:
///
/// ```rust
/// use plait::html;
///
/// let items = vec!["a", "b"];
///
/// let html = html! {
///     ul {
///         for item in &items {
///             li { (item) }
///         }
///     }
/// };
///
/// assert_eq!(html.to_string(), "<ul><li>a</li><li>b</li></ul>");
/// ```
///
/// # Nesting Fragments
///
/// Use `@(expr)` to embed any [`HtmlDisplay`] value without escaping:
///
/// ```rust
/// use plait::html;
///
/// let inner = html! { p { "Hello" } };
/// let outer = html! { div { @(&inner) } };
///
/// assert_eq!(outer.to_string(), "<div><p>Hello</p></div>");
/// ```
///
/// # Ownership and Borrowing
///
/// `html!` expands into a `move` closure that implements [`Fn`] - it must be callable more than once (e.g. via
/// [`Display::fmt`](core::fmt::Display::fmt)). Values used in the template are moved into the closure, but because
/// the closure is [`Fn`], its captures live behind a shared reference and cannot be moved out.
///
/// For [`Copy`] types like `&str`, `i32`, or `bool`, this is invisible - they are copied each time the closure runs.
/// For owned types like [`String`] or [`Vec`], you must explicitly borrow with `&` inside the template:
///
/// ```rust,compile_fail
/// use plait::html;
///
/// let name = String::from("World");
///
/// // ERROR: cannot move `name` out of the `Fn` closure
/// let fragment = html! { p { (name) } };
/// ```
///
/// Use `&` to borrow the captured value instead:
///
/// ```rust
/// use plait::html;
///
/// let name = String::from("World");
///
/// let fragment = html! { p { (&name) } };
///
/// assert_eq!(fragment.to_string(), "<p>World</p>");
/// ```
///
/// The same applies anywhere a value is used inside the template - element children, attribute values, loop
/// iterators, etc. When in doubt, borrow with `&`.
///
/// # Calling Components
///
/// Use `@ComponentName(props; attrs) { children }`. See [`component!`] for details.
pub use plait_macros::html;

/// Defines a reusable HTML component as a struct that implements [`Component`].
///
/// # Syntax
///
/// ```text
/// component! {
///     [pub] fn Name[<generics>](prop: Type, ...) [where ...] {
///         // html! body - can use #attrs and #children
///     }
/// }
/// ```
///
/// The macro generates a struct with the given name and a [`Component`] implementation. Inside the body you write the
/// same DSL as [`html!`], plus two special interpolations:
///
/// - `#attrs` - spreads any extra HTML attributes forwarded from the call site.
/// - `#children` - renders the child content passed between the component's braces.
///
/// # Props vs. attributes
///
/// When calling a component, props and extra HTML attributes are separated by a semicolon:
///
/// ```text
/// @Button(class: "primary"; id: "btn", disabled?: true) { "Click" }
/// //      ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// //           props              HTML attributes
/// ```
///
/// Extra HTML attributes are always optional - if none are needed, the semicolon can be omitted.
///
/// # Generics
///
/// Components support explicit generics, anonymous lifetimes (`&str` desugars automatically), `impl Trait`
/// parameters, and `where` clauses:
///
/// ```rust
/// use plait::{HtmlDisplay, ClassPart, component, html, classes};
///
/// // Anonymous lifetimes + impl Trait
/// component! {
///     fn NavLink(href: &str, label: &str, class: impl ClassPart, active: bool) {
///         a(href: href, class: classes!(
///             "nav-link", class,
///             if *active { "active" } else { "" },
///         )) {
///             (label)
///         }
///     }
/// }
///
/// // Explicit generics with where clause
/// component! {
///     fn Card<H, F>(header: H, footer: F) where H: HtmlDisplay, F: HtmlDisplay {
///         div(class: "card") {
///             div(class: "header") { @(header) }
///             div(class: "body") { #children }
///             div(class: "footer") { @(footer) }
///         }
///     }
/// }
/// ```
///
/// # Prop access
///
/// Props are received as references inside the component body. Primitive types like `bool` and `u32` should be
/// dereferenced with `*`:
///
/// ```rust
/// use plait::{component, html};
///
/// component! {
///     fn Badge(count: u32, visible: bool) {
///         if *visible {
///             span(class: "badge") { (count) }
///         }
///     }
/// }
///
/// let html = html! {
///     @Badge(count: 5, visible: true) {}
/// };
///
/// assert_eq!(html.to_string(), "<span class=\"badge\">5</span>");
/// ```
///
/// # Full example
///
/// ```rust
/// use plait::{component, html, classes, ClassPart};
///
/// component! {
///     pub fn Button(class: impl ClassPart) {
///         button(class: classes!("btn", class), #attrs) {
///             #children
///         }
///     }
/// }
///
/// let html = html! {
///     @Button(class: "primary"; id: "submit", disabled?: false) {
///         "Submit"
///     }
/// };
///
/// assert_eq!(
///     html.to_string(),
///     "<button class=\"btn primary\" id=\"submit\">Submit</button>",
/// );
/// ```
pub use plait_macros::component;

/// A convenience constant equal to `None::<&str>`.
///
/// Useful as a default value for [`ClassPart`], optional [`HtmlDisplay`] props when calling components or setting
/// attributes inside [`html!`].
///
/// # Example
///
/// ```rust
/// use plait::{component, html, classes, ClassPart, NONE};
///
/// component! {
///     fn Alert(class: impl ClassPart) {
///         div(class: classes!("alert", class)) { #children }
///     }
/// }
///
/// let html = html! {
///     @Alert(class: NONE) { "No extra class" }
/// };
///
/// assert_eq!(html.to_string(), "<div class=\"alert\">No extra class</div>");
/// ```
pub const NONE: Option<&str> = None;

pub use self::{
    classes::{ClassPart, Classes},
    component::Component,
    display::HtmlDisplay,
    fragment::HtmlFragment,
    maybe_attr::MaybeAttributeValue,
};
