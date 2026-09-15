//! Internal construction protocol for `@Component` calls.
//!
//! Required props are tracked in the type system. Defaulted props use an outer
//! Option so an explicitly supplied `None` is distinct from an omitted prop.
//! Defaults are evaluated only at build time and only for omitted props.
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{GenericParam, Generics, Ident, ext::IdentExt, parse_quote};

use crate::ast::ComponentDefinition;

pub fn component_props(component: &ComponentDefinition) -> TokenStream {
    let name = &component.ident;
    let visibility = &component.visibility;
    let builder = format_ident!("__PlaitProps{}", name.unraw());
    let generics = &component.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let required: Vec<_> = component
        .fields
        .iter()
        .filter(|f| f.default.is_none())
        .collect();
    let states: Vec<_> = (0..required.len())
        .map(|i| fresh_type_ident(generics, format!("__PlaitState{i}")))
        .collect();
    let mut builder_generics = generics.clone();
    let lifetimes = builder_generics.lifetimes().count();
    for (i, state) in states.iter().enumerate() {
        builder_generics
            .params
            .insert(lifetimes + i, parse_quote!(#state));
    }
    let (builder_impl_generics, builder_type_generics, builder_where) =
        builder_generics.split_for_impl();
    let state_types: Vec<_> = states.iter().map(|s| quote!(#s)).collect();
    let missing_types = vec![quote!(::plait::__private::Missing); states.len()];
    let initial_args = builder_args(generics, &missing_types);
    let mut marker = format_ident!("__plait_marker");
    while component.fields.iter().any(|f| f.ident.unraw() == marker) {
        marker = format_ident!("{}_", marker);
    }

    let mut declarations = Vec::new();
    let mut initializers = Vec::new();
    let mut setters = Vec::new();
    let mut traits = Vec::new();
    let mut bounds = Vec::new();
    let mut resolved = Vec::new();
    let mut required_index = 0;
    for field in &component.fields {
        let ident = &field.ident;
        let ty = &field.ty;
        let setter = format_ident!("__plait_set_{}", ident.unraw());
        if let Some(default) = &field.default {
            declarations.push(quote!(#ident: ::core::option::Option<#ty>));
            initializers.push(quote!(#ident: ::core::option::Option::None));
            setters.push(quote! {
                pub fn #setter(mut self, __plait_value: #ty) -> Self {
                    self.#ident = ::core::option::Option::Some(__plait_value);
                    self
                }
            });
            resolved.push(quote!(#ident: self.#ident.unwrap_or_else(|| #default)));
        } else {
            let state = &states[required_index];
            let required_trait = format_ident!("__PlaitRequired{}_{}", name.unraw(), ident.unraw());
            let message = format!(
                "missing required prop `{}` for component `{}`",
                ident.unraw(),
                name.unraw()
            );
            declarations.push(quote!(#ident: #state));
            initializers.push(quote!(#ident: ::plait::__private::Missing));
            traits.push(quote! {
                #[doc(hidden)]
                #[allow(non_camel_case_types)]
                #[diagnostic::on_unimplemented(message = #message)]
                #visibility trait #required_trait<T> {
                    fn __plait_take(self) -> T;
                }
                impl<T> #required_trait<T> for ::plait::__private::Provided<T> {
                    fn __plait_take(self) -> T { self.0 }
                }
            });
            bounds.push(quote!(#state: #required_trait<#ty>));
            resolved.push(quote!(#ident: #required_trait::__plait_take(self.#ident)));
            let mut next_states = state_types.clone();
            next_states[required_index] = quote!(::plait::__private::Provided<#ty>);
            let next_args = builder_args(generics, &next_states);
            let other_fields = component
                .fields
                .iter()
                .filter(|f| f.ident != *ident)
                .map(|f| {
                    let other = &f.ident;
                    quote!(#other: self.#other)
                });
            setters.push(quote! {
                pub fn #setter(self, __plait_value: #ty) -> #builder #next_args {
                    #builder {
                        #ident: ::plait::__private::Provided(__plait_value),
                        #(#other_fields,)*
                        #marker: ::core::marker::PhantomData,
                    }
                }
            });
            required_index += 1;
        }
    }
    let build_where = if bounds.is_empty() {
        quote!()
    } else {
        quote!(where #(#bounds),*)
    };

    quote! {
        #(#traits)*

        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #visibility struct #builder #builder_generics #builder_where {
            #(#declarations,)*
            #marker: ::core::marker::PhantomData<fn() -> #name #type_generics>,
        }

        impl #impl_generics #name #type_generics #where_clause {
            #[doc(hidden)]
            pub fn __plait_props() -> #builder #initial_args {
                #builder {
                    #(#initializers,)*
                    #marker: ::core::marker::PhantomData,
                }
            }
        }

        #[allow(non_snake_case, clippy::redundant_closure)]
        impl #builder_impl_generics #builder #builder_type_generics #builder_where {
            #(#setters)*

            pub fn __plait_build(self) -> #name #type_generics #build_where {
                #name { #(#resolved),* }
            }
        }
    }
}

/// Keep generated state parameters before user type/const parameters, which may
/// have defaults. Lifetimes must precede both groups.
fn builder_args(generics: &Generics, states: &[TokenStream]) -> TokenStream {
    let lifetimes = generics.lifetimes().map(|p| &p.lifetime);
    let other = generics.params.iter().filter_map(|p| match p {
        GenericParam::Lifetime(_) => None,
        GenericParam::Type(p) => {
            let ident = &p.ident;
            Some(quote!(#ident))
        }
        GenericParam::Const(p) => {
            let ident = &p.ident;
            Some(quote!(#ident))
        }
    });
    quote!(<#(#lifetimes,)* #(#states,)* #(#other,)*>)
}

fn fresh_type_ident(generics: &Generics, mut candidate: String) -> Ident {
    while generics.params.iter().any(|p| match p {
        GenericParam::Type(p) => p.ident.unraw() == candidate,
        GenericParam::Const(p) => p.ident.unraw() == candidate,
        GenericParam::Lifetime(_) => false,
    }) {
        candidate.push('_');
    }
    format_ident!("{candidate}")
}
