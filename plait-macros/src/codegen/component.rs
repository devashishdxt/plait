use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{ast::ComponentDefinition, buffer::InnerBuffer, codegen::desugar::desugar_fields};

pub fn component_impl(input: TokenStream) -> TokenStream {
    let mut component_definition: ComponentDefinition = match syn::parse2(input) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error(),
    };

    desugar_fields(
        &mut component_definition.fields,
        &mut component_definition.generics,
    );

    let component_struct = component_struct(&component_definition);
    let component_component_impl = component_component_impl(&component_definition);

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

fn component_component_impl(component: &ComponentDefinition) -> TokenStream {
    let ident = &component.ident;
    let (impl_generics, type_generics, where_clause) = component.generics.split_for_impl();

    let deconstruct = component_struct_deconstruct(&component);

    let writer = Ident::new("__plait_component", component.ident.span());

    let mut buffer = InnerBuffer::new(writer.clone());
    buffer.push_block(&component.body);
    buffer.flush_static_str();

    let statements = buffer.token_stream;
    let size_hint = buffer.size_hint;

    quote! {
        impl #impl_generics ::plait::Component for #ident #type_generics #where_clause {
            const SIZE_HINT: usize = #size_hint;

            fn render_component(
                &self,
                #writer: &mut (dyn ::core::fmt::Write + '_),
                attrs: impl ::core::ops::Fn(&mut (dyn ::core::fmt::Write + '_)) -> ::core::fmt::Result,
                children: impl ::core::ops::Fn(&mut (dyn ::core::fmt::Write + '_)) -> ::core::fmt::Result,
            ) -> ::core::fmt::Result {
                #deconstruct
                #statements

                Ok(())
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
