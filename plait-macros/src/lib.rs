mod ast;
mod buffer;
mod codegen;
mod parse;
mod utils;

use proc_macro::TokenStream;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    codegen::html_impl(input.into()).into()
}

#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    codegen::component_impl(input.into()).into()
}
