mod ast;
mod codegen;

use proc_macro::TokenStream;

#[proc_macro]
pub fn attrs(input: TokenStream) -> TokenStream {
    codegen::attrs_impl(input.into()).into()
}
