use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

use crate::{ast::Node, codegen::statements::push_statements_for_node};

struct HtmlInput {
    nodes: Vec<Node>,
    span: Span,
}

impl Parse for HtmlInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let span = input.span();

        let mut nodes = Vec::new();

        while !input.is_empty() {
            nodes.push(input.parse()?);
        }

        Ok(HtmlInput { nodes, span })
    }
}

pub fn html_impl(input: TokenStream) -> TokenStream {
    let html_input: HtmlInput = match syn::parse2(input) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error(),
    };

    let mut statements = Vec::new();

    let formatter = Ident::new("__plait_html_formatter", html_input.span);

    for node in html_input.nodes {
        push_statements_for_node(&mut statements, &formatter, node);
    }

    quote! {
        ::plait::HtmlFragment(|#formatter : &mut ::plait::HtmlFormatter<'_>| {
            #(#statements)*
        })
    }
}
