mod attributes;
mod error;
mod escape;
mod formatter;
mod html;
mod pre_escaped;
mod render;

pub use self::{
    attributes::Attributes,
    error::Error,
    escape::EscapeMode,
    formatter::HtmlFormatter,
    html::Html,
    pre_escaped::{DOCTYPE, PreEscaped},
    render::Render,
};

pub use plait_macros::{attrs, html, render};
