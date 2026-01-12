use crate::{EscapeMode, escape::escape_html};

use super::{Error, Html, PreEscaped};

/// Trait for rendering HTML content into an output.
pub trait Render {
    /// Renders the HTML content into the provided output.
    fn render(&self, output: &mut String, escape_mode: EscapeMode) -> Result<(), Error>;
}

impl Render for str {
    fn render(&self, output: &mut String, escape_mode: EscapeMode) -> Result<(), Error> {
        match escape_mode {
            EscapeMode::Raw => output.push_str(self),
            EscapeMode::Html => escape_html(output, self)?,
        }
        Ok(())
    }
}

impl Render for String {
    fn render(&self, output: &mut String, escape_mode: EscapeMode) -> Result<(), Error> {
        self.as_str().render(output, escape_mode)
    }
}

impl Render for PreEscaped<'_> {
    fn render(&self, output: &mut String, _escape_mode: EscapeMode) -> Result<(), Error> {
        output.push_str(self.0);
        Ok(())
    }
}

impl Render for Html {
    fn render(&self, output: &mut String, _escape_mode: EscapeMode) -> Result<(), Error> {
        output.push_str(&self.0);
        Ok(())
    }
}

macro_rules! impl_render_for_int {
    ($($ty:ty),*) => {
        $(
            impl Render for $ty {
                fn render(
                    &self,
                    output: &mut String,
                    _escape_mode: EscapeMode,
                ) -> Result<(), Error> {
                    let mut buf = itoa::Buffer::new();
                    output.push_str(buf.format(*self));
                    Ok(())
                }
            }
        )*
    };
}

impl_render_for_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

macro_rules! impl_render_for_float {
    ($($ty:ty),*) => {
        $(
            impl Render for $ty {
                fn render(
                    &self,
                    output: &mut String,
                    _escape_mode: EscapeMode,
                ) -> Result<(), Error> {
                    let mut buf = ryu::Buffer::new();
                    output.push_str(buf.format(*self));
                    Ok(())
                }
            }
        )*
    };
}

impl_render_for_float!(f32, f64);

impl Render for bool {
    fn render(&self, output: &mut String, _escape_mode: EscapeMode) -> Result<(), Error> {
        output.push_str(if *self { "true" } else { "false" });
        Ok(())
    }
}

impl Render for char {
    fn render(&self, output: &mut String, escape_mode: EscapeMode) -> Result<(), Error> {
        let mut buf = [0u8; 4];
        let s = self.encode_utf8(&mut buf);
        s.render(output, escape_mode)
    }
}

impl<T: Render + ?Sized> Render for &T {
    fn render(&self, output: &mut String, escape_mode: EscapeMode) -> Result<(), Error> {
        (*self).render(output, escape_mode)
    }
}
