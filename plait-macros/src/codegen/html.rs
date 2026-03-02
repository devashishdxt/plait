use proc_macro2::TokenStream;

use crate::{ast::Template, buffer::Buffer};

pub fn html_impl(input: TokenStream) -> TokenStream {
    let mut buffer = Buffer::new(&input);

    let html_input: Template = match syn::parse2(input) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error(),
    };

    buffer.push_block(&html_input.nodes);
    buffer.finalize_html()
}
