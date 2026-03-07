#![cfg_attr(docsrs, feature(doc_cfg))]
//! A modern, type-safe HTML templating library for Rust that embraces composition.
//!
//! Plait lets you write HTML directly in Rust using the [`html!`] macro, with compile-time validation, automatic
//! escaping, and a natural syntax that mirrors standard HTML and Rust control flow. Reusable components are defined
//! with the [`component!`] macro.
//!
//! # Quick start
//!
//! ```
//! use plait::{html, ToHtml};
//!
//! let name = "World";
//! let page = html! {
//!     div(class: "greeting") {
//!         h1 { "Hello, " (name) "!" }
//!     }
//! };
//!
//! assert_eq!(page.to_html(), r#"<div class="greeting"><h1>Hello, World!</h1></div>"#);
//! ```
//!
//! The [`html!`] macro returns an [`HtmlFragment`] that implements [`ToHtml`]. Call [`.to_html()`](ToHtml::to_html) to
//! get an [`Html`] value (a `String` wrapper that implements [`Display`](std::fmt::Display)).
//!
//! # Syntax reference
//!
//! ## Elements
//!
//! Write element names directly. Children go inside braces. Void elements (like `br`, `img`, `input`) use a semicolon
//! instead.
//!
//! ```
//! # use plait::{html, ToHtml};
//! let frag = html! {
//!     div {
//!         p { "Hello" }
//!         br;
//!         img(src: "/logo.png");
//!     }
//! };
//! ```
//!
//! Snake-case identifiers are automatically converted to kebab-case:
//!
//! ```
//! # use plait::{html, ToHtml};
//! // Renders as <my-element>...</my-element>
//! let frag = html! { my_element { "content" } };
//! assert_eq!(frag.to_html(), "<my-element>content</my-element>");
//! ```
//!
//! ## DOCTYPE
//!
//! Use `#doctype` to emit `<!DOCTYPE html>`:
//!
//! ```
//! # use plait::{html, ToHtml};
//! let page = html! {
//!     #doctype
//!     html {
//!         head { title { "My Page" } }
//!         body { "Hello" }
//!     }
//! };
//!
//! assert_eq!(page.to_html(), "<!DOCTYPE html><html><head><title>My Page</title></head><body>Hello</body></html>");
//! ```
//!
//! ## Text and expressions
//!
//! String literals are rendered as static text (HTML-escaped). Rust expressions inside parentheses are also
//! HTML-escaped by default. Use `#(expr)` for raw (unescaped) output.
//!
//! ```
//! # use plait::{html, ToHtml};
//! let user = "<script>alert('xss')</script>";
//! let frag = html! {
//!     "Static text "
//!     (user)              // escaped: &lt;script&gt;...
//!     #("<b>bold</b>")    // raw: <b>bold</b>
//! };
//! # assert_eq!(frag.to_html(), "Static text &lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;<b>bold</b>");
//! ```
//!
//! Expressions in `()` must implement [`RenderEscaped`]. Expressions in `#()` must implement [`RenderRaw`].
//!
//! ## Attributes
//!
//! Attributes go in parentheses after the element name.
//!
//! ```
//! # use plait::{html, ToHtml};
//! let frag = html! {
//!     // String value
//!     div(class: "container", id: "main") { "content" }
//!
//!     // Boolean attribute (no value) - always rendered
//!     button(disabled) { "Can't click" }
//!
//!     // Expression value (escaped)
//!     input(type: "text", value: ("hello"));
//!
//!     // Raw expression value (unescaped)
//!     div(class: #("raw-class")) {}
//! };
//! # assert_eq!(frag.to_html(), "<div class=\"container\" id=\"main\">content</div><button disabled>Can&#39;t click</button><input type=\"text\" value=\"hello\"><div class=\"raw-class\"></div>");
//! ```
//!
//! Underscore-to-hyphen conversion applies to attribute names too:
//!
//! ```
//! # use plait::{html, ToHtml};
//! // Renders as hx-target="body"
//! let frag = html! { div(hx_target: "body") {} };
//!
//! assert_eq!(frag.to_html(), "<div hx-target=\"body\"></div>");
//! ```
//!
//! Use string literals for attribute names that need special characters:
//!
//! ```
//! # use plait::{html, ToHtml};
//! let frag = html! { div("@click": "handler()") {} };
//!
//! assert_eq!(frag.to_html(), r#"<div @click="handler()"></div>"#);
//! ```
//!
//! ## Optional attributes
//!
//! Append `?` to the attribute name (before the `:`) to make it conditional. The attribute is only rendered when the
//! value is `Some(_)` (for [`Option`]) or `true` (for [`bool`]).
//!
//! ```
//! # use plait::{html, ToHtml};
//! let class = Some("active");
//! let disabled = false;
//!
//! let frag = html! {
//!     button(class?: class, disabled?: disabled) { "Click" }
//! };
//! assert_eq!(frag.to_html(), r#"<button class="active">Click</button>"#);
//! ```
//!
//! Values for `?` attributes must implement [`RenderMaybeAttributeEscaped`] (or [`RenderMaybeAttributeRaw`] when used
//! with `#()`).
//!
//! ## Control flow
//!
//! Standard Rust `if`/`else`, `if let`, `for`, and `match` work inside templates:
//!
//! ```
//! # use plait::{html, ToHtml};
//! let items = vec!["one", "two", "three"];
//! let show_header = true;
//!
//! let frag = html! {
//!     if show_header {
//!         h1 { "List" }
//!     }
//!
//!     ul {
//!         for item in items.iter() {
//!             li { (item) }
//!         }
//!     }
//! };
//!
//! # assert_eq!(frag.to_html(), r#"<h1>List</h1><ul><li>one</li><li>two</li><li>three</li></ul>"#);
//! ```
//!
//! ```
//! # use plait::{html, ToHtml};
//! let value = Some("hello");
//!
//! let frag = html! {
//!     if let Some(v) = value {
//!         span { (v) }
//!     } else {
//!         span { "nothing" }
//!     }
//! };
//!
//! # assert_eq!(frag.to_html(), r#"<span>hello</span>"#);
//! ```
//!
//! ```
//! # use plait::{html, ToHtml};
//! let tag = "div";
//!
//! let frag = html! {
//!     match tag {
//!         "div" => div { "a div" },
//!         "span" => span { "a span" },
//!         _ => "unknown"
//!     }
//! };
//!
//! # assert_eq!(frag.to_html(), r#"<div>a div</div>"#);
//! ```
//!
//! ## Let bindings
//!
//! Compute intermediate values within templates:
//!
//! ```
//! # use plait::{html, ToHtml};
//! let world = "World";
//!
//! let frag = html! {
//!     let len = world.len();
//!     "Length: " (len)
//! };
//! assert_eq!(frag.to_html(), "Length: 5");
//! ```
//!
//! ## Nesting fragments
//!
//! [`HtmlFragment`] implements [`RenderEscaped`], so fragments can be embedded in other fragments:
//!
//! ```
//! # use plait::{html, ToHtml};
//! let inner = html! { p { "inner content" } };
//! let outer = html! { div { (inner) } };
//! assert_eq!(outer.to_html(), "<div><p>inner content</p></div>");
//! ```
//!
//! # Components
//!
//! Components are reusable template functions defined with the [`component!`] macro:
//!
//! ```
//! use plait::{component, classes, Class};
//!
//! component! {
//!     pub fn Button(class: impl Class) {
//!         button(class: classes!("btn", class), #attrs) {
//!             #children
//!         }
//!     }
//! }
//! ```
//!
//! The macro generates a struct and a [`Component`] trait implementation. Components are
//! called with `@` syntax inside [`html!`]:
//!
//! ```
//! # use plait::{component, html, ToHtml, classes, Class};
//! # component! {
//! #     pub fn Button(class: impl Class) {
//! #         button(class: classes!("btn", class), #attrs) {
//! #             #children
//! #         }
//! #     }
//! # }
//! let page = html! {
//!     @Button(class: "primary"; id: "submit-btn", disabled?: false) {
//!         "Submit"
//!     }
//! };
//!
//! assert_eq!(
//!     page.to_html(),
//!     r#"<button class="btn primary" id="submit-btn">Submit</button>"#
//! );
//! ```
//!
//! In the component call, props appear before the `;`, and extra HTML attributes appear after. The component body uses
//! `#attrs` to spread those extra attributes and `#children` to render the child content.
//!
//! ## Shorthand props
//!
//! When a variable has the same name as a component prop, you can use shorthand syntax - just like Rust struct
//! initialization:
//!
//! ```
//! # use plait::{component, html, ToHtml, classes, Class};
//! # component! {
//! #     pub fn Button(class: impl Class) {
//! #         button(class: classes!("btn", class), #attrs) {
//! #             #children
//! #         }
//! #     }
//! # }
//! let class = "primary";
//!
//! // These are equivalent:
//! let a = html! { @Button(class: class) { "Click" } };
//! let b = html! { @Button(class) { "Click" } };
//!
//! assert_eq!(a.to_html(), b.to_html());
//! ```
//!
//! Shorthand and explicit props can be mixed freely:
//!
//! ```
//! # use plait::{component, html, ToHtml};
//! # component! {
//! #     pub fn UserCard(name: &str, role: &str) {
//! #         div { span { (name) } " - " span { (role) } }
//! #     }
//! # }
//! let name = "Alice";
//! let html = html! { @UserCard(name, role: "Admin") {} };
//!
//! assert_eq!(html.to_html(), "<div><span>Alice</span> - <span>Admin</span></div>");
//! ```
//!
//! ## Passing fragments as props
//!
//! Use [`PartialHtml`] as a prop bound to accept [`html!`] output as a component prop:
//!
//! ```
//! # use plait::{component, html, ToHtml, PartialHtml};
//! component! {
//!     pub fn Card(title: impl PartialHtml) {
//!         div(class: "card") {
//!             h1 { (title) }
//!             #children
//!         }
//!     }
//! }
//!
//! let page = html! {
//!     @Card(title: html! { span { "My Card" } }) {
//!         p { "Card body" }
//!     }
//! };
//! ```
//!
//! ## Primitive props
//!
//! Component props are received as references. For primitive types like `bool` or `u32`, dereference with `*` in the
//! component body:
//!
//! ```
//! # use plait::{component, html, ToHtml};
//! component! {
//!     pub fn Badge(count: u32, visible: bool) {
//!         if *visible {
//!             span(class: "badge") { (count) }
//!         }
//!     }
//! }
//! ```
//!
//! # CSS classes
//!
//! The [`classes!`] macro combines multiple class values, automatically skipping empty strings and `None` values:
//!
//! ```
//! # use plait::{html, ToHtml, classes};
//! let extra: Option<&str> = None;
//!
//! let frag = html! {
//!     div(class: classes!("base", "primary", extra)) {}
//! };
//! assert_eq!(frag.to_html(), r#"<div class="base primary"></div>"#);
//! ```
//!
//! Values passed to [`classes!`] must implement the [`Class`] trait. This is implemented for `&str`, `Option<T>` where
//! `T: Class`, and [`Classes<T>`](Classes).
//!
//! # Web framework integrations
//!
//! Plait provides optional integrations with popular Rust web frameworks. Both [`Html`] and [`HtmlFragment`] can be
//! returned directly from request handlers when the corresponding feature is enabled.
//!
//! Enable integrations by adding the feature flag to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! plait = { version = "0.8", features = ["axum"] }
//! ```
//!
//! Available features: `actix-web`, `axum`, `rocket`.
//!
//! ## axum
//!
//! [`Html`] and [`HtmlFragment`] implement
//! [`IntoResponse`](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html):
//!
//! ```ignore
//! use axum::{Router, routing::get};
//! use plait::{html, ToHtml};
//!
//! async fn index() -> plait::Html {
//!     html! {
//!         h1 { "Hello from plait!" }
//!     }.to_html()
//! }
//!
//! let app = Router::new().route("/", get(index));
//! ```
//!
//! You can also return an [`HtmlFragment`] directly without calling `.to_html()`:
//!
//! ```ignore
//! async fn index() -> impl axum::response::IntoResponse {
//!     plait::html! {
//!         h1 { "Hello from plait!" }
//!     }
//! }
//! ```
//!
//! ## actix-web
//!
//! [`Html`] and [`HtmlFragment`] implement
//! [`Responder`](https://docs.rs/actix-web/latest/actix_web/trait.Responder.html):
//!
//! ```ignore
//! use actix_web::{App, HttpServer, get};
//! use plait::{html, ToHtml};
//!
//! #[get("/")]
//! async fn index() -> plait::Html {
//!     html! {
//!         h1 { "Hello from plait!" }
//!     }.to_html()
//! }
//! ```
//!
//! ## rocket
//!
//! [`Html`] and [`HtmlFragment`] implement
//! [`Responder`](https://docs.rs/rocket/latest/rocket/response/trait.Responder.html):
//!
//! ```ignore
//! use rocket::get;
//! use plait::{html, ToHtml};
//!
//! #[get("/")]
//! fn index() -> plait::Html {
//!     html! {
//!         h1 { "Hello from plait!" }
//!     }.to_html()
//! }
//! ```
mod classes;
mod component;
mod fragment;
mod html;
mod maybe_attr;
mod render;
mod utils;

/// Generates an [`HtmlFragment`] from a template DSL.
///
/// The returned fragment implements [`ToHtml`] (call `.to_html()` to get an [`Html`] string) and [`RenderEscaped`] (so
/// it can be embedded inside other `html!` calls).
///
/// See the [crate-level documentation](crate) for a full syntax reference.
///
/// # Quick example
///
/// ```
/// use plait::{html, ToHtml};
///
/// let name = "World";
/// let page = html! {
///     div(class: "greeting") {
///         h1 { "Hello, " (name) "!" }
///     }
/// };
///
/// assert_eq!(page.to_html(), r#"<div class="greeting"><h1>Hello, World!</h1></div>"#);
/// ```
///
/// # Syntax summary
///
/// | Syntax                                  | Description                                             |
/// |-----------------------------------------|---------------------------------------------------------|
/// | `tag { ... }`                           | Element with children                                   |
/// | `tag(attrs) { ... }`                    | Element with attributes and children                    |
/// | `tag;`                                  | Void element (e.g. `br;`, `img(src: "...");`)           |
/// | `"text"`                                | Static text (HTML-escaped)                              |
/// | `(expr)`                                | Escaped expression ([`RenderEscaped`])                  |
/// | `#(expr)`                               | Raw expression ([`RenderRaw`])                          |
/// | `#doctype`                              | `<!DOCTYPE html>`                                       |
/// | `attr: "value"`                         | Static string attribute                                 |
/// | `attr: (expr)`                          | Escaped expression attribute                            |
/// | `attr: #(expr)`                         | Raw expression attribute                                |
/// | `attr`                                  | Boolean attribute (always present)                      |
/// | `attr?: expr`                           | Conditional attribute ([`RenderMaybeAttributeEscaped`]) |
/// | `attr?: #(expr)`                        | Conditional raw attribute ([`RenderMaybeAttributeRaw`]) |
/// | `if` / `else` / `if let`                | Conditional rendering                                   |
/// | `for pat in iter { ... }`               | Loop                                                    |
/// | `match expr { ... }`                    | Pattern matching                                        |
/// | `let x = expr;`                         | Let binding                                             |
/// | `@Component(props; attrs) { children }` | Component call                                          |
pub use plait_macros::html;

/// Defines a reusable HTML component (struct + [`Component`] trait implementation).
///
/// See the [crate-level documentation](crate#components) for full details.
///
/// # Syntax
///
/// ```
/// # use plait::{component, classes, Class};
/// component! {
///     pub fn Button(class: impl Class) {
///         button(class: classes!("btn", class), #attrs) {
///             #children
///         }
///     }
/// }
/// ```
///
/// The macro generates a struct named `Button` with public fields, and implements [`Component`] for it.
///
/// # Special tokens
///
/// - `#attrs` — renders extra HTML attributes passed at the call site (after `;`).
/// - `#children` — renders child content from inside the component's braces.
///
/// # Field desugaring
///
/// - `&str` → auto-generated lifetime `&'plait_N str`
/// - `impl Trait` → generic type parameter `P_N: Trait`
///
/// # Calling
///
/// ```
/// # use plait::{component, html, classes, Class, ToHtml};
/// # component! {
/// #     pub fn Button(class: impl Class) {
/// #         button(class: classes!("btn", class), #attrs) {
/// #             #children
/// #         }
/// #     }
/// # }
/// let html = html! {
///     @Button(class: "primary"; id: "btn1", disabled?: false) {
///         "Click me"
///     }
/// };
///
/// assert_eq!(html.to_html(), "<button class=\"btn primary\" id=\"btn1\">Click me</button>");
/// ```
///
/// Props go before `;`, extra HTML attributes go after.
///
/// ## Shorthand props
///
/// When a variable has the same name as a prop, you can omit the value - just like Rust struct initialization
/// shorthand:
///
/// ```
/// # use plait::{component, html, classes, Class, ToHtml};
/// # component! {
/// #     pub fn Button(class: impl Class) {
/// #         button(class: classes!("btn", class), #attrs) {
/// #             #children
/// #         }
/// #     }
/// # }
/// let class = "primary";
///
/// let html = html! {
///     @Button(class) { "Click" }
/// };
///
/// assert_eq!(html.to_html(), "<button class=\"btn primary\">Click</button>");
/// ```
pub use plait_macros::component;

pub use self::{
    classes::{Class, Classes},
    component::Component,
    fragment::{HtmlFragment, PartialHtml},
    html::{Html, ToHtml},
    maybe_attr::{RenderMaybeAttributeEscaped, RenderMaybeAttributeRaw},
    render::{RenderEscaped, RenderRaw},
};
