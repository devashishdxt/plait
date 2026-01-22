//! A fast, type-safe HTML templating library for Rust.
//!
//! Plait provides compile-time HTML generation with a familiar, JSX-like syntax. Templates are checked at compile
//! time and generate efficient code with minimal runtime overhead.
//!
//! # Quick Start
//!
//! ```rust
//! use plait::html;
//!
//! let name = "World";
//! let output = html!(
//!     div class="greeting" {
//!         h1 { "Hello, " (name) "!" }
//!     }
//! );
//!
//! assert_eq!(&*output, r#"<div class="greeting"><h1>Hello, World!</h1></div>"#);
//! ```
//!
//! # Macros
//!
//! Plait provides four main macros:
//!
//! - [`html!`] - Creates an [`Html`] value from a template
//! - [`component!`] - Creates a reusable component that renders lazily
//! - [`render!`] - Renders content to an existing [`HtmlFormatter`]
//! - [`attrs!`] - Creates an [`Attributes`] collection
//!
//! # Template Syntax
//!
//! ## Elements
//!
//! Elements are written as `name { children }` for normal elements or `name;` for void elements:
//!
//! ```rust
//! use plait::html;
//!
//! let output = html!(
//!     div {
//!         p { "A paragraph" }
//!         br;
//!         input type="text" name="field";
//!     }
//! );
//!
//! assert_eq!(&*output, r#"<div><p>A paragraph</p><br><input type="text" name="field"></div>"#);
//! ```
//!
//! ## Attributes
//!
//! Attributes support several value types:
//!
//! ```rust
//! use plait::html;
//!
//! let class_name = "container";
//! let maybe_id: Option<&str> = Some("main");
//! let is_disabled = true;
//!
//! let output = html!(
//!     div
//!         class="literal"              // Literal string
//!         data-value=(class_name)      // Dynamic expression
//!         id=[maybe_id]                // Optional (renders if Some)
//!         disabled?[is_disabled]       // Boolean (renders if true)
//!     {
//!         "content"
//!     }
//! );
//!
//! assert_eq!(&*output, r#"<div class="literal" data-value="container" id="main" disabled>content</div>"#);
//! ```
//!
//! ## Dynamic Content
//!
//! Expressions in parentheses are escaped by default:
//!
//! ```rust
//! use plait::html;
//!
//! let user_input = "<script>alert('xss')</script>";
//!
//! let output = html!(
//!     div { (user_input) }
//! );
//!
//! // Content is safely escaped
//! assert!(!output.contains("<script>"));
//! ```
//!
//! Use `: raw` to include pre-escaped content:
//!
//! ```rust
//! use plait::html;
//!
//! let trusted_html = "<strong>Bold</strong>";
//!
//! let output = html!(
//!     div { (trusted_html : raw) }
//! );
//!
//! assert!(output.contains("<strong>"));
//! ```
//!
//! ## Control Flow
//!
//! ### Conditionals
//!
//! ```rust
//! use plait::html;
//!
//! let show = true;
//! let value: Option<&str> = Some("hello");
//!
//! let output = html!(
//!     div {
//!         @if show {
//!             span { "Visible" }
//!         }
//!
//!         @if let Some(v) = value {
//!             span { (v) }
//!         } @else {
//!             span { "No value" }
//!         }
//!     }
//! );
//!
//! assert_eq!(&*output, r#"<div><span>Visible</span><span>hello</span></div>"#);
//! ```
//!
//! ### Loops
//!
//! ```rust
//! use plait::html;
//!
//! let items = vec!["one", "two", "three"];
//!
//! let output = html!(
//!     ul {
//!         @for item in &items {
//!             li { (item) }
//!         }
//!     }
//! );
//!
//! assert_eq!(&*output, r#"<ul><li>one</li><li>two</li><li>three</li></ul>"#);
//! ```
//!
//! ### Match Expressions
//!
//! ```rust
//! use plait::html;
//!
//! enum Status { Active, Inactive }
//! let status = Status::Active;
//!
//! let output = html!(
//!     span {
//!         @match status {
//!             Status::Active => "Online",
//!             Status::Inactive => "Offline",
//!         }
//!     }
//! );
//!
//! assert_eq!(&*output, r#"<span>Online</span>"#);
//! ```
//!
//! # Custom Components
//!
//! Use the [`component!`] macro to create reusable components as functions:
//!
//! ```rust
//! use plait::{Render, component};
//!
//! fn button(label: &str, primary: bool) -> impl Render + '_ {
//!     let class = if primary { "btn btn-primary" } else { "btn" };
//!     component! {
//!         button class=(class) { (label) }
//!     }
//! }
//!
//! // Use in templates
//! let output = plait::html!(
//!     div { (button("Click me", true)) }
//! );
//!
//! assert_eq!(&*output, r#"<div><button class="btn btn-primary">Click me</button></div>"#);
//! ```
//!
//! For components with owned data, use `(&value)` to borrow:
//!
//! ```rust
//! use plait::{Render, component};
//!
//! fn greeting(message: String) -> impl Render {
//!     component! {
//!         span { (&message) }
//!     }
//! }
//! ```
//!
//! For more control, implement the [`Render`] trait directly. See the [`Render`] documentation for details.
//!
//! # Safety
//!
//! Plait automatically escapes dynamic content to prevent XSS vulnerabilities. The [`Html`] and [`PreEscaped`] types
//! represent content that is already safe and will not be escaped again.

mod attributes;
mod error;
mod escape;
mod formatter;
mod html;
mod lazy;
mod pre_escaped;
mod render;

pub use self::{
    attributes::Attributes,
    error::Error,
    escape::EscapeMode,
    formatter::HtmlFormatter,
    html::Html,
    lazy::LazyRender,
    pre_escaped::{DOCTYPE, PreEscaped},
    render::Render,
};

pub use plait_macros::{attrs, component, html, render};
