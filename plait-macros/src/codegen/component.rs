use proc_macro2::TokenStream;
use quote::quote;

/// Implementation of the `component!` procedural macro.
///
/// Wraps the input template in a `LazyRender` closure that defers rendering
/// until the component is embedded in a parent template. The closure captures
/// the template content and renders it using the parent's formatter.
pub fn component_impl(input: TokenStream) -> TokenStream {
    quote! {
        ::plait::LazyRender(move |__plait_component_formatter: &mut ::plait::HtmlFormatter| {
            ::plait::render!(__plait_component_formatter, { #input });
        })
    }
}
