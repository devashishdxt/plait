use std::fmt;

use crate::{Html, RenderEscaped, ToHtml};

pub struct HtmlFragment<F>
where
    F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
{
    f: F,
    size_hint: usize,
}

impl<F> HtmlFragment<F>
where
    F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
{
    #[doc(hidden)]
    pub fn new(f: F, size_hint: usize) -> Self {
        HtmlFragment { f, size_hint }
    }

    fn to_string(&self) -> String {
        let mut buffer = String::with_capacity(self.size_hint);
        (self.f)(&mut buffer).unwrap();
        buffer
    }
}

impl<F> RenderEscaped for HtmlFragment<F>
where
    F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
{
    fn render_escaped(&self, f: &mut (dyn fmt::Write + '_)) -> fmt::Result {
        (self.f)(f)
    }
}

impl<F> ToHtml for HtmlFragment<F>
where
    F: Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
{
    fn to_html(&self) -> Html {
        Html::new_unchecked(self.to_string())
    }
}
