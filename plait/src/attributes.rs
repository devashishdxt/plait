//! Hidden macro support, referenced by generated code as `::plait::__attrs`.
//! Rendering stays in a closure; only names need a trait for const evaluation.
use std::{fmt, marker::PhantomData};

/// A compile-time name tree. Values and rendering are deliberately separate.
pub struct Names {
    pub local: &'static [&'static str],
    pub inherited: &'static [Names],
}

impl Names {
    pub const fn contains(&self, name: &str) -> bool {
        let mut i = 0;
        while i < self.local.len() {
            if self.local[i].eq_ignore_ascii_case(name) {
                return true;
            }
            i += 1;
        }
        i = 0;
        while i < self.inherited.len() {
            if self.inherited[i].contains(name) {
                return true;
            }
            i += 1;
        }
        false
    }
}

pub trait Metadata {
    const NAMES: Names;
}

impl Metadata for () {
    const NAMES: Names = Names {
        local: &[],
        inherited: &[],
    };
}

impl<L, I> Metadata for (L, I)
where
    L: Metadata,
    I: Metadata,
{
    const NAMES: Names = Names {
        local: &[],
        inherited: &[L::NAMES, I::NAMES],
    };
}

/// An ordinary rendering closure tagged with compile-time names.
pub struct Bundle<M, F> {
    render: F,
    marker: PhantomData<fn() -> M>,
}

impl<M, F> Bundle<M, F>
where
    M: Metadata,
    F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
{
    pub fn new(_: M, render: F) -> Self {
        Self {
            render,
            marker: PhantomData,
        }
    }

    /// Used only when the new bundle includes `#attrs`. The closure retains
    /// all rendering captures; the name tag adds no value evaluation or borrow.
    pub fn with_names<L, G>(&self, _: L, render: G) -> Bundle<(L, M), G>
    where
        L: Metadata,
        G: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
    {
        Bundle {
            render,
            marker: PhantomData,
        }
    }

    pub fn render(&self, writer: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        (self.render)(writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct Local;
    impl Metadata for Local {
        const NAMES: Names = Names {
            local: &["TYPE", "x-on:click", "type"],
            inherited: &[],
        };
    }

    fn contains<M, F>(_: &Bundle<M, F>, name: &str) -> bool
    where
        M: Metadata,
    {
        M::NAMES.contains(name)
    }

    #[test]
    fn metadata_is_composable_case_insensitive_and_never_renders() {
        assert!(!<()>::NAMES.contains("type"));
        let evaluations = Cell::new(0);
        let local = Bundle::new(Local, |writer| {
            evaluations.set(evaluations.get() + 1);
            writer.write_str(" value")
        });
        let inherited = local.with_names((), |writer| {
            local.render(writer)?;
            local.render(writer)
        });
        for bundle_has_name in [
            contains(&local, "type"),
            contains(&&inherited, "TyPe"),
            contains(&inherited, "x-on:click"),
        ] {
            assert!(bundle_has_name);
        }
        assert!(!contains(&inherited, "id"));
        assert!(!contains(&inherited, "x_on:click"));
        assert_eq!(evaluations.get(), 0);
        let mut output = String::new();
        inherited.render(&mut output).unwrap();
        assert_eq!(output, " value value");
        assert_eq!(evaluations.get(), 2);
    }
}
