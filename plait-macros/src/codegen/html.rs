use proc_macro2::TokenStream;
use quote::quote;

/// Implementation of the `html!` procedural macro.
///
/// Wraps the input template in boilerplate that creates an [`Html`](plait::Html)
/// output and [`HtmlFormatter`](plait::HtmlFormatter), then delegates to
/// `render!` for the actual content generation.
pub fn html_impl(input: TokenStream) -> TokenStream {
    quote! {
        {
            let mut __spool_output = ::plait::Html::new();
            let mut __spool_formatter_owned = ::plait::HtmlFormatter::new(&mut __spool_output);
            let __spool_formatter_borrowed = &mut __spool_formatter_owned;

            ::plait::render!(__spool_formatter_borrowed, { #input });

            __spool_output
        }
    }
}
