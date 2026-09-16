use std::fmt;

use crate::__attrs::{Bundle, Metadata};

/// Trait for reusable HTML components. Used by macro-generated code. Do NOT
/// implement this manually.
#[doc(hidden)]
pub trait Component {
    /// Renders the component, writing HTML into `f`.
    ///
    /// * `attrs` - typed bundle of extra HTML attribute names and their lazy
    ///   rendering closure from the call site.
    /// * `children` - closure that writes child content from the call site.
    fn render_component<M>(
        &self,
        f: &mut (dyn fmt::Write + '_),
        attrs: &Bundle<M, impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result>,
        children: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
    ) -> fmt::Result
    where
        M: Metadata;
}

/// Implementation details shared by macro-generated components.
#[doc(hidden)]
pub mod props {
    pub struct Missing;
    pub struct Provided<T>(pub T);
    pub struct Required;
    pub struct Resolved;

    pub trait Factory {
        type Output;
        fn make(self) -> Self::Output;
    }

    impl<F, T> Factory for F
    where
        F: FnOnce() -> T,
    {
        type Output = T;

        fn make(self) -> T {
            self()
        }
    }

    pub trait Resolve<F> {
        type Output;
        fn resolve(self, factory: F) -> Self::Output;
    }

    impl<F> Resolve<F> for Missing
    where
        F: Factory,
    {
        type Output = F::Output;

        fn resolve(self, factory: F) -> Self::Output {
            factory.make()
        }
    }

    // A supplied value never invokes the default, even for Provided(None).
    // Required deliberately does not implement Factory, so Missing cannot
    // resolve it.
    impl<T, F> Resolve<F> for Provided<T> {
        type Output = T;

        fn resolve(self, _: F) -> T {
            self.0
        }
    }
}

impl<T> Component for &T
where
    T: Component,
{
    fn render_component<M>(
        &self,
        f: &mut (dyn fmt::Write + '_),
        attrs: &Bundle<M, impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result>,
        children: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
    ) -> fmt::Result
    where
        M: Metadata,
    {
        (**self).render_component(f, attrs, children)
    }
}
