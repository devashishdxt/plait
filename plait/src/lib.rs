//! A lightweight, type-safe HTML templating library for Rust.
//!
//! Plait provides a macro-based DSL for writing HTML directly in Rust code with compile-time validation and automatic
//! escaping. It's designed for building server-side rendered HTML with minimal runtime overhead.
//!
//! # Quick Start
//!
//! ```rust
//! use plait::{html, render};
//!
//! let name = "World";
//! let html = render(html! {
//!     div(class: "greeting") {
//!         h1 { "Hello, " (name) "!" }
//!     }
//! });
//!
//! assert_eq!(html, "<div class=\"greeting\"><h1>Hello, World!</h1></div>");
//! ```
//!
//! # The `html!` Macro
//!
//! The [`html!`] macro provides a concise syntax for writing HTML:
//!
//! ```rust
//! use plait::{html, render};
//!
//! let html = render(html! {
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
//! });
//! ```
//!
//! ## Expressions
//!
//! Use parentheses to embed expressions. Content is automatically HTML-escaped:
//!
//! ```rust
//! use plait::{html, render};
//!
//! let user_input = "<script>alert('xss')</script>";
//! let html = render(html! { p { (user_input) } });
//!
//! assert_eq!(html, "<p>&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;</p>");
//! ```
//!
//! For raw, unescaped content, prefix with `#`:
//!
//! ```rust
//! use plait::{html, render};
//!
//! let trusted_html = "<strong>Bold</strong>";
//! let html = render(html! { div { #(trusted_html) } });
//!
//! assert_eq!(html, "<div><strong>Bold</strong></div>");
//! ```
//!
//! ## Attributes
//!
//! Attributes support several forms:
//!
//! ```rust
//! use plait::{html, render};
//!
//! let class = Some("active");
//! let disabled = true;
//!
//! let html = render(html! {
//!     // Boolean attribute (no value)
//!     button(checked) { "Checked" }
//!
//!     // Optional attribute with `?:` - included when Some or true
//!     div(class?: class) { "Has class" }
//!     button(disabled?: disabled) { "Disabled" }
//!
//!     // Attribute names with underscores become hyphens
//!     div(hx_target: "body") {}  // renders as hx-target="body"
//!
//!     // String attribute names for special characters
//!     div("@click": "handler()") {}
//! });
//! ```
//!
//! ## Control Flow
//!
//! Standard Rust control flow works naturally:
//!
//! ```rust
//! use plait::{html, render};
//!
//! let show = true;
//! let items = vec!["a", "b", "c"];
//! let variant = "primary";
//! let user = Some("Alice");
//!
//! let html = render(html! {
//!     // Conditionals
//!     if show {
//!         p { "Visible" }
//!     } else {
//!         p { "Hidden" }
//!     }
//!
//!     // if let with else
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
//! });
//! ```
//!
//! # Components
//!
//! Create reusable components using the [`component!`] macro:
//!
//! ```rust
//! use plait::{component, html, classes, render};
//!
//! component! {
//!     fn Button<'a>(class: &'a str) {
//!         button(class: classes!("btn", class), #attrs) {
//!             #children
//!         }
//!     }
//! }
//!
//! let html = render(html! {
//!     // Component props before `;`, HTML attributes after
//!     @Button(class: "primary"; id: "submit-btn", disabled?: false) {
//!         "Click me"
//!     }
//! });
//!
//! assert_eq!(html, "<button class=\"btn primary\" id=\"submit-btn\">Click me</button>");
//! ```
//!
//! Inside components, `#attrs` spreads additional HTML attributes and `#children` renders the component's children.
//!
//! ## Passing HTML as props
//!
//! Components can accept [`html!`] fragments as props using the [`ToHtml`] trait:
//!
//! ```rust
//! use plait::{ToHtml, component, html, render};
//!
//! component! {
//!     fn Card<T>(title: T) where T: ToHtml {
//!         div(class: "card") {
//!             h1 { (title) }
//!             #children
//!         }
//!     }
//! }
//!
//! let html = render(html! {
//!     @Card(title: html! { span(class: "highlight") { "My Title" } }) {
//!         p { "Card content" }
//!     }
//! });
//!
//! assert_eq!(
//!     html,
//!     "<div class=\"card\"><h1><span class=\"highlight\">My Title</span></h1><p>Card content</p></div>"
//! );
//! ```
//!
//! # URL Safety
//!
//! URL attributes (`href`, `src`, `action`, etc.) are automatically validated. Dangerous schemes like `javascript:`
//! are stripped:
//!
//! ```rust
//! use plait::{html, render};
//!
//! let html = render(html! {
//!     a(href: "javascript:alert('xss')") { "Click" }
//! });
//!
//! assert_eq!(html, "<a>Click</a>");  // href removed
//! ```
//!
//! Use `#(...)` for raw URLs when you trust the source.
//!
//! # Merging CSS Classes
//!
//! Use [`classes!`] to combine multiple class values into a single space-separated string. Empty strings and
//! `None` values are automatically skipped:
//!
//! ```rust
//! use plait::{component, html, classes, render};
//!
//! component! {
//!     fn Button<'a>(variant: Option<&'a str>) {
//!         button(class: classes!("btn", variant), #attrs) {
//!             #children
//!         }
//!     }
//! }
//!
//! let html = render(html! {
//!     @Button(variant: Some("btn-primary")) { "Click me" }
//! });
//!
//! assert_eq!(html, "<button class=\"btn btn-primary\">Click me</button>");
//! ```
//!
//! # Performance
//!
//! For better performance when output size is predictable, use [`render_with_capacity`] to pre-allocate the buffer:
//!
//! ```rust
//! use plait::{html, render_with_capacity};
//!
//! let html = render_with_capacity(4096, html! {
//!     div { "Content" }
//! });
//! ```
mod classes;
mod component;
mod formatter;
mod fragment;
mod html;
mod maybe_attr;
mod to_html;
mod url;

pub use plait_macros::{component, html};

pub use self::{
    classes::{ClassPart, Classes},
    component::Component,
    formatter::HtmlFormatter,
    fragment::HtmlFragment,
    html::Html,
    maybe_attr::MaybeAttr,
    to_html::{ToHtml, ToHtmlRaw},
};

/// Renders any [`ToHtml`] value to an [`Html`] string.
///
/// This function creates an [`Html`] buffer and renders the content into it. Typically used with the [`html!`] macro,
/// but accepts any type implementing [`ToHtml`].
///
/// # Examples
///
/// ```rust
/// use plait::{html, render};
///
/// let html = render(html! {
///     div(class: "container") {
///         "Hello, World!"
///     }
/// });
///
/// assert_eq!(html, "<div class=\"container\">Hello, World!</div>");
/// ```
pub fn render(content: impl ToHtml) -> Html {
    let mut output = Html::new();
    let mut f = HtmlFormatter::new(&mut output);
    content.render_to(&mut f);

    output
}

/// Renders any [`ToHtml`] value to an [`Html`] string with a pre-allocated buffer capacity.
///
/// This function is similar to [`render`], but pre-allocates the internal string buffer with the specified capacity.
/// Use this when you know the approximate size of the output to avoid reallocations and improve performance.
///
/// # Examples
///
/// ```rust
/// use plait::{html, render_with_capacity};
///
/// let html = render_with_capacity(1024, html! {
///     div(class: "card") {
///         h1 { "Title" }
///         p { "Content goes here..." }
///     }
/// });
///
/// assert_eq!(html, "<div class=\"card\"><h1>Title</h1><p>Content goes here...</p></div>");
/// ```
pub fn render_with_capacity(capacity: usize, content: impl ToHtml) -> Html {
    let mut output = Html::with_capacity(capacity);
    let mut f = HtmlFormatter::new(&mut output);
    content.render_to(&mut f);

    output
}
