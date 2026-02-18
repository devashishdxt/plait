use proc_macro2::Span;
use syn::{
    GenericParam, Generics, Ident, Lifetime, LifetimeParam, Type, TypeImplTrait, TypeParam,
    TypePath,
    visit_mut::{self, VisitMut},
};

use crate::ast::ComponentDefinitionField;

/// Desugars anonymous lifetimes and `impl Trait` in component fields into explicit generic
/// parameters.
///
/// After this function returns:
/// - Every `&str` or `&'_ str` in field types has been replaced with `&'plait_N str`
/// - Every `impl Trait` in field types has been replaced with a type parameter `P_N`
/// - The corresponding lifetime and type parameters have been added to `generics`
pub fn desugar_fields(fields: &mut [ComponentDefinitionField], generics: &mut Generics) {
    let mut lifetimes = CollectLifetimes::new();
    let mut impl_traits = CollectImplTraits::new();

    for field in fields.iter_mut() {
        lifetimes.visit_type_mut(&mut field.ty);
        impl_traits.visit_type_mut(&mut field.ty);
    }

    // Prepend lifetime params before existing params
    let existing_params: Vec<GenericParam> = generics.params.iter().cloned().collect();
    generics.params.clear();

    for lifetime in lifetimes.elided {
        generics
            .params
            .push(GenericParam::Lifetime(LifetimeParam::new(lifetime)));
    }

    for param in existing_params {
        generics.params.push(param);
    }

    // Append type params after existing params
    for type_param in impl_traits.type_params {
        generics.params.push(GenericParam::Type(type_param));
    }
}

/// Walks field types and replaces anonymous/elided lifetimes with named ones.
///
/// - `&str` becomes `&'plait_0 str`
/// - `&'_ str` becomes `&'plait_0 str`
/// - `&'a str` is left alone (already named)
/// - `&&str` gets two lifetimes: `&'plait_0 &'plait_1 str`
struct CollectLifetimes {
    elided: Vec<Lifetime>,
}

impl CollectLifetimes {
    fn new() -> Self {
        Self { elided: Vec::new() }
    }

    fn next_lifetime(&mut self, span: Span) -> Lifetime {
        let name = format!("'plait_{}", self.elided.len());
        let lifetime = Lifetime::new(&name, span);
        self.elided.push(lifetime.clone());
        lifetime
    }

    fn visit_opt_lifetime(&mut self, span: Span, lifetime: &mut Option<Lifetime>) {
        match lifetime {
            None => *lifetime = Some(self.next_lifetime(span)),
            Some(lifetime) => self.visit_lifetime(lifetime),
        }
    }

    fn visit_lifetime(&mut self, lifetime: &mut Lifetime) {
        if lifetime.ident == "_" {
            *lifetime = self.next_lifetime(lifetime.span());
        }
    }
}

impl VisitMut for CollectLifetimes {
    fn visit_type_reference_mut(&mut self, ty: &mut syn::TypeReference) {
        self.visit_opt_lifetime(ty.and_token.span, &mut ty.lifetime);
        visit_mut::visit_type_reference_mut(self, ty);
    }

    fn visit_generic_argument_mut(&mut self, arg: &mut syn::GenericArgument) {
        if let syn::GenericArgument::Lifetime(lifetime) = arg {
            self.visit_lifetime(lifetime);
        }
        visit_mut::visit_generic_argument_mut(self, arg);
    }
}

/// Walks field types and replaces `impl Trait` with named type parameters.
///
/// - `impl ClassPart` becomes `P0` (with `P0: ClassPart` added to generics)
/// - `impl Display + Debug` becomes `P0` (with `P0: Display + Debug`)
/// - Each `impl Trait` occurrence gets its own parameter
struct CollectImplTraits {
    type_params: Vec<TypeParam>,
}

impl CollectImplTraits {
    fn new() -> Self {
        Self {
            type_params: Vec::new(),
        }
    }

    fn next_type_param(&mut self, impl_trait: &TypeImplTrait) -> Type {
        let index = self.type_params.len();
        let ident = Ident::new(&format!("P{index}"), impl_trait.impl_token.span);

        let mut type_param = TypeParam::from(ident.clone());
        type_param.bounds = impl_trait.bounds.clone();

        self.type_params.push(type_param);

        Type::Path(TypePath {
            qself: None,
            path: ident.into(),
        })
    }
}

impl VisitMut for CollectImplTraits {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        if let Type::ImplTrait(impl_trait) = ty {
            *ty = self.next_type_param(impl_trait);
            // No need to recurse into the replacement (it's a simple path)
            return;
        }

        visit_mut::visit_type_mut(self, ty);
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote;

    use super::*;

    fn desugar(input: proc_macro2::TokenStream) -> (Vec<ComponentDefinitionField>, Generics) {
        let mut fields: Vec<ComponentDefinitionField> = Vec::new();

        // Parse as fn args: "name: Type, name2: Type2"
        let item: syn::ItemFn = syn::parse2(quote! {
            fn test(#input) {}
        })
        .unwrap();

        for arg in item.sig.inputs {
            if let syn::FnArg::Typed(pat_type) = arg {
                let ident = if let syn::Pat::Ident(pat_ident) = *pat_type.pat {
                    pat_ident.ident
                } else {
                    panic!("expected ident pattern");
                };
                fields.push(ComponentDefinitionField {
                    ident,
                    ty: *pat_type.ty,
                });
            }
        }

        let mut generics: Generics = parse_quote!();
        desugar_fields(&mut fields, &mut generics);
        (fields, generics)
    }

    fn type_to_string(ty: &Type) -> String {
        quote!(#ty).to_string()
    }

    fn generics_to_string(generics: &Generics) -> String {
        if generics.params.is_empty() {
            String::new()
        } else {
            quote!(#generics).to_string()
        }
    }

    #[test]
    fn test_anonymous_lifetime() {
        let (fields, generics) = desugar(quote! { x: &str });

        assert_eq!(type_to_string(&fields[0].ty), "& 'plait_0 str");
        assert_eq!(generics_to_string(&generics), "< 'plait_0 >");
    }

    #[test]
    fn test_underscore_lifetime() {
        let (fields, generics) = desugar(quote! { x: &'_ str });

        assert_eq!(type_to_string(&fields[0].ty), "& 'plait_0 str");
        assert_eq!(generics_to_string(&generics), "< 'plait_0 >");
    }

    #[test]
    fn test_explicit_lifetime_unchanged() {
        let (fields, generics) = desugar(quote! { x: &'a str });

        assert_eq!(type_to_string(&fields[0].ty), "& 'a str");
        // No new lifetime params generated
        assert_eq!(generics_to_string(&generics), "");
    }

    #[test]
    fn test_nested_references() {
        let (fields, generics) = desugar(quote! { x: &&str });

        assert_eq!(type_to_string(&fields[0].ty), "& 'plait_0 & 'plait_1 str");
        assert_eq!(generics_to_string(&generics), "< 'plait_0 , 'plait_1 >");
    }

    #[test]
    fn test_multiple_fields_with_refs() {
        let (fields, generics) = desugar(quote! { a: &str, b: &str });

        assert_eq!(type_to_string(&fields[0].ty), "& 'plait_0 str");
        assert_eq!(type_to_string(&fields[1].ty), "& 'plait_1 str");
        assert_eq!(generics_to_string(&generics), "< 'plait_0 , 'plait_1 >");
    }

    #[test]
    fn test_impl_trait() {
        let (fields, generics) = desugar(quote! { x: impl Display });

        assert_eq!(type_to_string(&fields[0].ty), "P0");
        assert_eq!(generics_to_string(&generics), "< P0 : Display >");
    }

    #[test]
    fn test_impl_trait_multiple_bounds() {
        let (fields, generics) = desugar(quote! { x: impl Display + Debug });

        assert_eq!(type_to_string(&fields[0].ty), "P0");
        assert_eq!(generics_to_string(&generics), "< P0 : Display + Debug >");
    }

    #[test]
    fn test_multiple_impl_traits() {
        let (fields, generics) = desugar(quote! { a: impl Display, b: impl Debug });

        assert_eq!(type_to_string(&fields[0].ty), "P0");
        assert_eq!(type_to_string(&fields[1].ty), "P1");
        assert_eq!(
            generics_to_string(&generics),
            "< P0 : Display , P1 : Debug >"
        );
    }

    #[test]
    fn test_combined_lifetime_and_impl_trait() {
        let (fields, generics) = desugar(quote! { a: &str, b: impl ClassPart });

        assert_eq!(type_to_string(&fields[0].ty), "& 'plait_0 str");
        assert_eq!(type_to_string(&fields[1].ty), "P0");
        assert_eq!(
            generics_to_string(&generics),
            "< 'plait_0 , P0 : ClassPart >"
        );
    }

    #[test]
    fn test_existing_generics_preserved() {
        let mut fields = Vec::new();

        let item: syn::ItemFn = syn::parse2(quote! {
            fn test(x: &str, y: impl Display) {}
        })
        .unwrap();

        for arg in item.sig.inputs {
            if let syn::FnArg::Typed(pat_type) = arg {
                let ident = if let syn::Pat::Ident(pat_ident) = *pat_type.pat {
                    pat_ident.ident
                } else {
                    panic!("expected ident pattern");
                };
                fields.push(ComponentDefinitionField {
                    ident,
                    ty: *pat_type.ty,
                });
            }
        }

        let mut generics: Generics = parse_quote!(<'a, T: Debug>);
        desugar_fields(&mut fields, &mut generics);

        // Order: elided lifetimes, then existing params, then impl trait params
        assert_eq!(
            generics_to_string(&generics),
            "< 'plait_0 , 'a , T : Debug , P0 : Display >"
        );
    }

    #[test]
    fn test_option_with_anonymous_lifetime() {
        let (fields, generics) = desugar(quote! { x: Option<&str> });

        assert_eq!(type_to_string(&fields[0].ty), "Option < & 'plait_0 str >");
        assert_eq!(generics_to_string(&generics), "< 'plait_0 >");
    }

    #[test]
    fn test_no_desugaring_needed() {
        let (fields, generics) = desugar(quote! { x: String, y: u32 });

        assert_eq!(type_to_string(&fields[0].ty), "String");
        assert_eq!(type_to_string(&fields[1].ty), "u32");
        assert_eq!(generics_to_string(&generics), "");
    }
}
