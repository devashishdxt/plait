use proc_macro2::TokenStream;
use quote::quote;

pub fn html_impl(input: TokenStream) -> TokenStream {
    quote! {
        {
            let mut __spool_output = ::spool::Html::new();
            let mut __spool_formatter_owned = ::spool::HtmlFormatter::new(&mut __spool_output);
            let __spool_formatter_borrowed = &mut __spool_formatter_owned;

            ::spool::render!(__spool_formatter_borrowed, { #input });

            __spool_output
        }
    }
}
