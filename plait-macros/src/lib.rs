//! Procedural macros for the [`plait`](https://docs.rs/plait) HTML templating library.
//!
//! This crate provides the [`html!`] and [`component!`] macros. You should depend on the `plait` crate directly -
//! these macros are re-exported from there with full documentation.

mod ast;
mod buffer;
mod codegen;
mod parse;
mod utils;

use proc_macro::TokenStream;

/// See [`plait::html!`](https://docs.rs/plait/latest/plait/macro.html.html) for full documentation.
///
/// # Example
///
/// ```ignore
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
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    codegen::html_impl(input.into()).into()
}

/// See [`plait::component!`](https://docs.rs/plait/latest/plait/macro.component.html) for full documentation.
///
/// # Example
///
/// ```ignore
/// use plait::{component, classes, Class};
///
/// component! {
///     pub fn Button(class: impl Class) {
///         button(class: classes!("btn", class), #attrs) {
///             #children
///         }
///     }
/// }
/// ```
#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    codegen::component_impl(input.into()).into()
}
