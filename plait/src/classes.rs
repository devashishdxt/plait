use std::fmt;

use crate::RenderEscaped;

pub trait Class {
    fn should_skip(&self) -> bool;

    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result;
}

impl<T> Class for &T
where
    T: Class + ?Sized,
{
    fn should_skip(&self) -> bool {
        (**self).should_skip()
    }

    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        (**self).render_escaped(f)
    }
}

impl<T> Class for Option<T>
where
    T: Class,
{
    fn should_skip(&self) -> bool {
        match self {
            Some(value) => value.should_skip(),
            None => true,
        }
    }

    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        match self {
            Some(value) => value.render_escaped(f),
            None => Ok(()),
        }
    }
}

impl Class for str {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }

    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        RenderEscaped::render_escaped(self, f)
    }
}

pub struct Classes<T>(pub T);

impl<T> Class for Classes<T>
where
    T: Class,
{
    fn should_skip(&self) -> bool {
        self.0.should_skip()
    }

    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        self.0.render_escaped(f)
    }
}

impl<T> RenderEscaped for Classes<T>
where
    T: Class,
{
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        Class::render_escaped(&self.0, f)
    }
}

impl<T> fmt::Display for Classes<T>
where
    T: Class,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Class::render_escaped(&self, f)
    }
}

macro_rules! impl_class_for_tuple {
    ($($idx:tt: $T:ident),+) => {
        impl<$($T: $crate::Class),+> $crate::Class for ($($T,)+) {
            fn should_skip(&self) -> bool {
                true $( && $crate::Class::should_skip(&self.$idx) )+
            }

            #[allow(unused_assignments)]
            fn render_escaped(&self, f: &mut (dyn ::core::fmt::Write + '_)) -> ::core::fmt::Result {
                let mut needs_space = false;

                $(
                    if !$crate::Class::should_skip(&self.$idx) {
                        if needs_space {
                            ::core::fmt::Write::write_char(f, ' ')?;
                        }
                        $crate::Class::render_escaped(&self.$idx, f)?;
                        needs_space = true;
                    }
                )+

                Ok(())
            }
        }
    };
}

impl_class_for_tuple!(0: T0);
impl_class_for_tuple!(0: T0, 1: T1);
impl_class_for_tuple!(0: T0, 1: T1, 2: T2);
impl_class_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3);
impl_class_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4);
impl_class_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5);
impl_class_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6);
impl_class_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7);

#[macro_export]
macro_rules! classes {
    ($($class:expr),+ $(,)?) => {
        $crate::Classes(($($class,)+))
    };
}
