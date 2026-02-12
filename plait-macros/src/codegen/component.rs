use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

use crate::{
    ast::ComponentDefinition, codegen::statements::push_statements_for_node,
    desugar::desugar_fields,
};

struct ComponentInput {
    component: ComponentDefinition,
    span: Span,
}

impl Parse for ComponentInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(ComponentInput {
            component: input.parse()?,
            span: input.span(),
        })
    }
}

pub fn component_impl(input: TokenStream) -> TokenStream {
    let mut component_input: ComponentInput = match syn::parse2(input) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error(),
    };

    desugar_fields(
        &mut component_input.component.fields,
        &mut component_input.component.generics,
    );

    let component_struct = component_struct(&component_input.component);

    let component_component_impl =
        component_component_impl(component_input.component, component_input.span);

    quote! {
        #component_struct
        #component_component_impl
    }
}

fn component_struct(component: &ComponentDefinition) -> TokenStream {
    let attributes = &component.attributes;
    let visibility = &component.visibility;
    let name = &component.ident;

    let mut field_statements = Vec::new();

    for field in component.fields.iter() {
        let field_name = &field.ident;
        let field_type = &field.ty;

        field_statements.push(quote! {
            pub #field_name: #field_type
        });
    }

    let generics = &component.generics;
    let where_clause = &generics.where_clause;

    let out = quote! {
        #(#attributes)*
        #visibility struct #name #generics #where_clause {
            #(#field_statements),*
        }
    };

    out
}

fn component_component_impl(component: ComponentDefinition, span: Span) -> TokenStream {
    let ident = &component.ident;
    let (impl_generics, type_generics, where_clause) = component.generics.split_for_impl();

    let deconstruct = component_struct_deconstruct(&component);

    let formatter = Ident::new("f", span);

    let mut statements = Vec::new();

    for node in component.body {
        push_statements_for_node(&mut statements, &formatter, node);
    }

    quote! {
        impl #impl_generics ::plait::Component for #ident #type_generics #where_clause {
            fn render(
                self,
                #formatter : &mut ::plait::HtmlFormatter<'_>,
                attrs: impl ::core::ops::FnOnce(&mut ::plait::HtmlFormatter<'_>),
                children: impl ::core::ops::FnOnce(&mut ::plait::HtmlFormatter<'_>),
            ) {
                #deconstruct
                #(#statements)*
            }
        }
    }
}

fn component_struct_deconstruct(component: &ComponentDefinition) -> TokenStream {
    if component.fields.is_empty() {
        return quote! {};
    }

    let mut fields = Vec::new();

    for field in component.fields.iter() {
        let ident = &field.ident;
        fields.push(quote! {
            #ident
        });
    }

    let ident = &component.ident;

    quote! {
        let #ident { #(#fields),* } = self;
    }
}
