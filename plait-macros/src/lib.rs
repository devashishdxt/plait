mod ast;
mod codegen;

use proc_macro::TokenStream;

#[proc_macro]
pub fn attrs(input: TokenStream) -> TokenStream {
    codegen::attrs_impl(input.into()).into()
}

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    codegen::html_impl(input.into()).into()
}

#[proc_macro]
pub fn render(input: TokenStream) -> TokenStream {
    codegen::render_impl(input.into()).into()
}
