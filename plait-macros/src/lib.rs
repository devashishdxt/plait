//! Procedural macros for the plait HTML templating library.
//!
//! This crate provides three macros for generating HTML:
//!
//! - [`html!`] - Creates an [`Html`](plait::Html) value from a template
//! - [`render!`] - Renders content to an existing [`HtmlFormatter`](plait::HtmlFormatter)
//! - [`attrs!`] - Creates an [`Attributes`](plait::Attributes) collection
//!
//! These macros are re-exported by the main `plait` crate and should typically
//! be used from there rather than directly from this crate.

mod ast;
mod codegen;

use proc_macro::TokenStream;

/// Creates an [`Attributes`](plait::Attributes) collection from a declarative syntax.
///
/// # Syntax
///
/// ```text
/// attrs!(
///     name="literal"           // Literal string value
///     name=(expr)              // Dynamic value from expression
///     name=[optional_expr]     // Optional value (only rendered if Some)
///     name?[bool_expr]         // Boolean attribute (rendered if true)
///     name                     // Boolean attribute (always rendered)
///     ..(spread_expr)          // Spread attributes from another collection
/// )
/// ```
///
/// # Example
///
/// ```rust,ignore
/// use plait::attrs;
///
/// let class_name = "container";
/// let attrs = attrs!(
///     id="main"
///     class=(class_name)
///     disabled?[true]
/// );
/// ```
#[proc_macro]
pub fn attrs(input: TokenStream) -> TokenStream {
    codegen::attrs_impl(input.into()).into()
}

/// Creates an [`Html`](plait::Html) value from a declarative template.
///
/// This is the primary macro for creating HTML content. It returns an owned
/// `Html` value containing the rendered template.
///
/// # Syntax
///
/// ```text
/// html!(
///     element attr="value" {
///         "text content"
///         (dynamic_expr)
///         nested_element { ... }
///         @if condition { ... }
///         @for item in items { ... }
///         @match expr { ... }
///     }
/// )
/// ```
///
/// # Elements
///
/// Elements are written as `name { children }` for normal elements or `name;`
/// for void elements (like `<br>`, `<input>`).
///
/// # Attributes
///
/// - `attr="literal"` - Literal string value
/// - `attr=(expr)` - Dynamic value from expression
/// - `attr=[optional]` - Optional value (only rendered if Some)
/// - `attr?[bool]` - Boolean attribute (rendered without value if true)
///
/// # Content
///
/// - `"text"` - Literal text (not escaped, included as-is)
/// - `(expr)` - Dynamic expression (escaped by default)
/// - `(expr : raw)` - Dynamic expression without escaping
/// - `{ ... }` - Fragment containing multiple nodes
///
/// # Control Flow
///
/// - `@if condition { ... }` - Conditional rendering
/// - `@if condition { ... } @else { ... }` - If-else
/// - `@if let pattern = expr { ... }` - Pattern matching
/// - `@for pattern in expr { ... }` - Iteration
/// - `@match expr { pattern => ..., }` - Match expression
///
/// # Example
///
/// ```rust,ignore
/// use plait::html;
///
/// let name = "World";
/// let items = vec!["one", "two", "three"];
///
/// let output = html!(
///     div class="greeting" {
///         h1 { "Hello, " (name) "!" }
///         ul {
///             @for item in &items {
///                 li { (item) }
///             }
///         }
///     }
/// );
/// ```
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    codegen::html_impl(input.into()).into()
}

/// Renders content to an existing [`HtmlFormatter`](plait::HtmlFormatter).
///
/// Unlike [`html!`], which creates a new `Html` value, `render!` writes directly
/// to a formatter. This is useful for implementing the rendering of custom types
/// or for building HTML incrementally.
///
/// # Syntax
///
/// ```text
/// render!(formatter, { content })
/// ```
///
/// The content syntax is the same as [`html!`].
///
/// # Example
///
/// ```rust,ignore
/// use plait::{Html, HtmlFormatter, render};
///
/// let mut output = Html::new();
/// let mut fmt = HtmlFormatter::new(&mut output);
///
/// render!(fmt, {
///     div { "Hello, world!" }
/// });
/// ```
#[proc_macro]
pub fn render(input: TokenStream) -> TokenStream {
    codegen::render_impl(input.into()).into()
}
