use plait::{Component, html};
use std::fmt;

struct Handwritten;
impl Component for Handwritten {
    fn render_component(
        &self,
        f: &mut (dyn fmt::Write + '_),
        attrs: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
        children: impl Fn(&mut (dyn fmt::Write + '_)) -> fmt::Result,
    ) -> fmt::Result {
        attrs(f)?;
        children(f)
    }
}

fn main() {
    let _ = html! { @Handwritten {} };
}
