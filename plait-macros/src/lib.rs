//! Proc macros for the [`plait`](https://docs.rs/plait) crate.
//!
//! This crate provides the [`html!`] and [`component!`] proc macros. You should depend on `plait` directly rather
//! than using this crate — both macros are re-exported from there.
mod ast;
mod codegen;

use proc_macro::TokenStream;

/// Produces an `HtmlFragment` from a concise markup DSL.
///
/// The returned value implements [`Display`](core::fmt::Display) and `HtmlDisplay`.
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    codegen::html_impl(input.into()).into()
}

/// Defines a reusable HTML component as a struct that implements `Component`.
#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    codegen::component_impl(input.into()).into()
}
