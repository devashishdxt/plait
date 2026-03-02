use syn::parse::{Parse, ParseStream};

use crate::ast::Template;

impl Parse for Template {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            nodes.push(input.parse()?);
        }

        Ok(Self { nodes })
    }
}
