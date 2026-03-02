mod classes;
mod component;
mod fragment;
mod html;
mod maybe_attr;
mod render;
mod utils;

pub use plait_macros::{component, html};

pub use self::{
    classes::{Class, Classes},
    component::{Component, component_size_hint},
    fragment::HtmlFragment,
    html::{Html, ToHtml},
    maybe_attr::{RenderMaybeAttributeEscaped, RenderMaybeAttributeRaw},
    render::{RenderEscaped, RenderRaw},
};
