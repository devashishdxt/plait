use std::collections::HashSet;

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    GenericParam, Generics, Ident, Index, Type,
    ext::IdentExt,
    parse_quote,
    visit_mut::{self, VisitMut},
};

use crate::{
    ast::ComponentDefinition,
    buffer::InnerBuffer,
    codegen::desugar::{desugar_fields_avoiding as desugar_fields, desugar_lifetimes},
};

pub fn component_impl(input: TokenStream) -> TokenStream {
    let mut reserved = HashSet::new();
    collect_identifiers(input.clone(), &mut reserved);
    let component: ComponentDefinition = match syn::parse2(input) {
        Ok(component) => component,
        Err(error) => return error.to_compile_error(),
    };
    ComponentGenerator::new(&component, &reserved).generate()
}

/// Prefix setters so a prop named `__plait_props` or `__plait_resolve` cannot
/// collide with the entry/resolution methods. Raw identifiers name the same prop.
pub fn setter_name(ident: &Ident) -> Ident {
    format_ident!("__plait_set_{}", ident.unraw(), span = ident.span())
}

/// Shared names and type arguments for the separate generated implementations.
struct ComponentGenerator<'a> {
    component: &'a ComponentDefinition,
    reserved: &'a HashSet<String>,
    user_args: Vec<TokenStream>,
    props: Ident,
    factories: Ident,
    states: Vec<Ident>,
    factory_types: Vec<Ident>,
    indices: Vec<Index>,
}

impl<'a> ComponentGenerator<'a> {
    fn new(component: &'a ComponentDefinition, reserved: &'a HashSet<String>) -> Self {
        let field_count = component.fields.len();
        Self {
            component,
            reserved,
            user_args: arguments(&component.generics),
            props: fresh_ident("__PlaitProps", reserved),
            factories: fresh_ident("__PlaitFactories", reserved),
            states: (0..field_count)
                .map(|i| fresh_ident(&format!("__PlaitState{i}"), reserved))
                .collect(),
            factory_types: (0..field_count)
                .map(|i| fresh_ident(&format!("__PlaitFactory{i}"), reserved))
                .collect(),
            indices: (0..field_count).map(Index::from).collect(),
        }
    }

    fn generate(&self) -> TokenStream {
        let storage = self.storage();
        let constructor = self.constructor();
        let setters = self.setters();
        let resolver = self.resolver();
        let renderer = self.renderer();
        quote! {
            #storage
            #constructor
            #setters
            #resolver
            #renderer
        }
    }

    fn storage(&self) -> TokenStream {
        let component = self.component;
        let name = &component.ident;
        let attributes = &component.attributes;
        let visibility = &component.visibility;
        let props = &self.props;
        let factories = &self.factories;
        let mut generics = component.generics.clone();
        generics.params.push(parse_quote!(#props = ()));
        generics.params.push(parse_quote!(#factories = ()));
        let where_clause = &generics.where_clause;
        let marker = generic_marker(&component.generics);

        // One type carries prop states during construction and values after resolution.
        quote! {
            #(#attributes)*
            #visibility struct #name #generics #where_clause {
                __plait_values: #props,
                __plait_factories: #factories,
                __plait_marker: ::core::marker::PhantomData<fn() -> #marker>,
            }
        }
    }

    fn constructor(&self) -> TokenStream {
        let component = self.component;
        let name = &component.ident;
        let user = &component.generics;
        let user_args = &self.user_args;
        let (impl_generics, _, where_clause) = user.split_for_impl();
        let missing: Vec<_> = self
            .indices
            .iter()
            .map(|_| quote!(::plait::__props::Missing))
            .collect();

        // Only declared user generics live on the initial specialization. Keep
        // anonymous bounds in factory outputs rather than guessing their types.
        let mut fields = component.fields.clone();
        let mut generics = user.clone();
        desugar_lifetimes(&mut fields, &mut generics, self.reserved);
        let lifetimes: Vec<_> = generics
            .lifetimes()
            .filter(|p| !user.lifetimes().any(|u| u.lifetime == p.lifetime))
            .collect();
        let (outputs, values): (Vec<_>, Vec<_>) = fields
            .iter()
            .map(|field| {
                if let Some(default) = &field.default {
                    let ty = &field.ty;
                    let mut inferred_ty = ty.clone();
                    InferAnonymous.visit_type_mut(&mut inferred_ty);
                    (
                        quote!(impl ::plait::__props::Factory<Output = #ty>),
                        quote!(move || -> #inferred_ty { #default }),
                    )
                } else {
                    let required = quote!(::plait::__props::Required);
                    (required.clone(), required)
                }
            })
            .unzip();

        quote! {
            impl #impl_generics #name<#(#user_args,)* (), ()> #where_clause {
                #[doc(hidden)]
                pub fn __plait_props<#(#lifetimes),*>() -> #name<
                    #(#user_args,)* (#(#missing,)*), (#(#outputs,)*)
                > {
                    #name {
                        __plait_values: (#(#missing,)*),
                        __plait_factories: (#(#values,)*),
                        __plait_marker: ::core::marker::PhantomData,
                    }
                }
            }
        }
    }

    fn setters(&self) -> TokenStream {
        let name = &self.component.ident;
        let user_args = &self.user_args;
        let states = &self.states;
        let factories = &self.factories;
        let mut generics = self.component.generics.clone();
        for state in states {
            generics.params.push(parse_quote!(#state));
        }
        generics.params.push(parse_quote!(#factories));
        let (impl_generics, _, where_clause) = generics.split_for_impl();
        let setters = (0..self.component.fields.len()).map(|i| self.setter(i));

        quote! {
            impl #impl_generics #name<#(#user_args,)* (#(#states,)*), #factories> #where_clause {
                #(#setters)*
            }
        }
    }

    fn setter(&self, index: usize) -> TokenStream {
        let name = &self.component.ident;
        let user = &self.component.generics;
        let user_args = &self.user_args;
        let factories = &self.factories;
        let value = fresh_ident("__plait_value", self.reserved);
        let mut field = self.component.fields[index].clone();
        let setter = setter_name(&field.ident);
        let mut generics = user.clone();
        desugar_fields(
            std::slice::from_mut(&mut field),
            &mut generics,
            self.reserved,
        );
        let ty = &field.ty;
        let method_params = added_parameters(&generics, user);
        let output_states = self.states.iter().enumerate().map(|(i, state)| {
            if i == index {
                quote!(::plait::__props::Provided<#ty>)
            } else {
                quote!(#state)
            }
        });
        let output_values = self.indices.iter().enumerate().map(|(i, field_index)| {
            if i == index {
                quote!(::plait::__props::Provided(#value))
            } else {
                quote!(self.__plait_values.#field_index)
            }
        });

        quote! {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            pub fn #setter <#(#method_params),*> (self, #value: #ty)
                -> #name<#(#user_args,)* (#(#output_states,)*), #factories>
            {
                #name {
                    __plait_values: (#(#output_values,)*),
                    __plait_factories: self.__plait_factories,
                    __plait_marker: ::core::marker::PhantomData,
                }
            }
        }
    }

    fn resolver(&self) -> TokenStream {
        let name = &self.component.ident;
        let user_args = &self.user_args;
        let states = &self.states;
        let factory_types = &self.factory_types;
        let mut generics = self.component.generics.clone();
        for (state, factory) in states.iter().zip(factory_types) {
            generics.params.push(parse_quote!(#state));
            generics.params.push(parse_quote!(#factory));
            generics
                .make_where_clause()
                .predicates
                .push(parse_quote!(#state: ::plait::__props::Resolve<#factory>));
        }
        let (impl_generics, _, where_clause) = generics.split_for_impl();
        let outputs = states.iter().zip(factory_types).map(
            |(state, factory)| quote!(<#state as ::plait::__props::Resolve<#factory>>::Output),
        );
        let resolutions = self.indices.iter().map(|index| quote!(
            ::plait::__props::Resolve::resolve(self.__plait_values.#index, self.__plait_factories.#index)
        ));

        quote! {
            impl #impl_generics #name<#(#user_args,)* (#(#states,)*), (#(#factory_types,)*)> #where_clause {
                #[doc(hidden)]
                pub fn __plait_resolve(self) -> #name<#(#user_args,)* (#(#outputs,)*), ::plait::__props::Resolved> {
                    #name {
                        __plait_values: (#(#resolutions,)*),
                        __plait_factories: ::plait::__props::Resolved,
                        __plait_marker: ::core::marker::PhantomData,
                    }
                }
            }
        }
    }

    fn renderer(&self) -> TokenStream {
        let component = self.component;
        let name = &component.ident;
        let user_args = &self.user_args;
        let mut fields = component.fields.clone();
        let mut generics = component.generics.clone();
        desugar_fields(&mut fields, &mut generics, self.reserved);
        let (impl_generics, _, where_clause) = generics.split_for_impl();
        let types = fields.iter().map(|f| &f.ty);
        let bindings = component.fields.iter().map(|f| &f.ident);
        let writer = fresh_ident("__plait_component", self.reserved);
        let attrs = fresh_ident("__plait_attrs", self.reserved);
        let attrs_type = fresh_ident("__PlaitAttributes", self.reserved);
        let validations = component.reserved_attrs.iter().map(|reserved| {
            // Escape format braces in literal attribute names, retaining the
            // exact name in the static diagnostic without const formatting.
            let message = format!(
                "reserved attribute `{}` on component `{}`",
                reserved.value(),
                name.unraw(),
            )
            .replace('{', "{{")
            .replace('}', "}}");
            quote_spanned! {reserved.span()=>
                const {
                    ::core::assert!(
                        !<#attrs_type as ::plait::__attrs::Metadata>::NAMES.contains(#reserved),
                        #message,
                    );
                }
            }
        });
        let children = fresh_ident("__plait_children", self.reserved);
        let aliases = self.callback_aliases(&attrs, &children);
        let mut buffer = InnerBuffer::new(writer.clone());
        buffer.component_callbacks = Some((attrs.clone(), children.clone()));
        buffer.push_block(&component.body);
        buffer.flush_static_str();
        let statements = buffer.token_stream;

        quote! {
            impl #impl_generics ::plait::Component for #name<#(#user_args,)* (#(#types,)*), ::plait::__props::Resolved> #where_clause {
                fn render_component<#attrs_type>(
                    &self,
                    #writer: &mut (dyn ::core::fmt::Write + '_),
                    #attrs: &::plait::__attrs::Bundle<#attrs_type, impl ::core::ops::Fn(&mut (dyn ::core::fmt::Write + '_)) -> ::core::fmt::Result>,
                    #children: impl ::core::ops::Fn(&mut (dyn ::core::fmt::Write + '_)) -> ::core::fmt::Result,
                ) -> ::core::fmt::Result
                where
                    #attrs_type: ::plait::__attrs::Metadata,
                {
                    #(#validations)*
                    #aliases
                    let (#(#bindings,)*) = &self.__plait_values;
                    #statements
                    Ok(())
                }
            }
        }
    }

    fn callback_aliases(&self, attrs: &Ident, children: &Ident) -> TokenStream {
        // Nested html! expressions retain their declaration-scope callbacks.
        // Borrow the attribute bundle itself (not a rendering adapter) so its
        // associated metadata survives nested templates and repeated references.
        // Direct directives use hygienic names even for props named attrs/children.
        [("attrs", attrs), ("children", children)]
            .into_iter()
            .filter(|(alias, _)| {
                !self
                    .component
                    .fields
                    .iter()
                    .any(|field| field.ident.unraw() == *alias)
                    && !self
                        .component
                        .generics
                        .const_params()
                        .any(|param| param.ident.unraw() == *alias)
            })
            .map(|(alias, internal)| {
                let alias = Ident::new(alias, Span::call_site());
                quote!(let #alias = &#internal;)
            })
            .collect()
    }
}

fn collect_identifiers(tokens: TokenStream, reserved: &mut HashSet<String>) {
    for token in tokens {
        match token {
            TokenTree::Ident(ident) => {
                reserved.insert(ident.unraw().to_string());
            }
            TokenTree::Group(group) => collect_identifiers(group.stream(), reserved),
            _ => {}
        }
    }
}

fn fresh_ident(base: &str, reserved: &HashSet<String>) -> Ident {
    let mut candidate = base.to_owned();
    while reserved.contains(&candidate) {
        candidate.push('_');
    }
    Ident::new(&candidate, Span::mixed_site())
}

fn arguments(generics: &Generics) -> Vec<TokenStream> {
    generics
        .params
        .iter()
        .map(|param| match param {
            GenericParam::Lifetime(param) => {
                let lifetime = &param.lifetime;
                quote!(#lifetime)
            }
            GenericParam::Type(param) => {
                let ident = &param.ident;
                quote!(#ident)
            }
            GenericParam::Const(param) => {
                let ident = &param.ident;
                quote!({ #ident })
            }
        })
        .collect()
}

fn generic_marker(generics: &Generics) -> TokenStream {
    let marker = generics.params.iter().filter_map(|param| match param {
        GenericParam::Lifetime(param) => {
            let lifetime = &param.lifetime;
            Some(quote!(&#lifetime ()))
        }
        GenericParam::Type(param) => {
            let ident = &param.ident;
            Some(quote!(*const #ident))
        }
        _ => None,
    });
    quote!((#(#marker,)*))
}

fn added_parameters<'a>(generics: &'a Generics, user: &Generics) -> Vec<&'a GenericParam> {
    generics
        .params
        .iter()
        .filter(|p| {
            !user.params.iter().any(|u| match (u, *p) {
                (GenericParam::Lifetime(u), GenericParam::Lifetime(p)) => u.lifetime == p.lifetime,
                (GenericParam::Type(u), GenericParam::Type(p)) => u.ident == p.ident,
                (GenericParam::Const(u), GenericParam::Const(p)) => u.ident == p.ident,
                _ => false,
            })
        })
        .collect()
}

// Closure return annotations allow `_` but not `impl Trait`. Keep concrete
// portions for expected-type coercions; the opaque factory signature enforces
// each anonymous bound without capturing unrelated generic parameters.
struct InferAnonymous;
impl VisitMut for InferAnonymous {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        if matches!(ty, Type::ImplTrait(_)) {
            *ty = parse_quote!(_);
        } else {
            visit_mut::visit_type_mut(self, ty);
        }
    }
}
