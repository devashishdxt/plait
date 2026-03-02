use std::fmt;

pub trait Component {
    const SIZE_HINT: usize;

    fn render_component(
        &self,
        f: &mut (dyn fmt::Write + '_),
        attrs: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
        children: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
    ) -> fmt::Result;
}

impl<T> Component for &T
where
    T: Component,
{
    const SIZE_HINT: usize = T::SIZE_HINT;

    fn render_component(
        &self,
        f: &mut (dyn fmt::Write + '_),
        attrs: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
        children: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
    ) -> fmt::Result {
        (**self).render_component(f, attrs, children)
    }
}

pub const fn component_size_hint<T>(_: &T) -> usize
where
    T: Component,
{
    T::SIZE_HINT
}
