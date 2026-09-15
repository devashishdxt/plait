#![allow(dead_code)]

use plait::{PartialHtml, RenderEscaped, classes, component};

// Intentionally no Default impl: defaults belong to the component declaration.
#[derive(Clone, Copy)]
pub enum Appearance {
    Solid,
    Outline,
}

impl Appearance {
    fn class(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Outline => "outline",
        }
    }
}

#[derive(Clone, Copy)]
pub enum ButtonKind {
    Button,
    Submit,
}

impl ButtonKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
        }
    }
}

component! {
    pub fn Button(
        kind: ButtonKind = ButtonKind::Button,
        appearance: Appearance = Appearance::Solid,
        disabled: bool = false,
        class: &str = "",
    ) {
        button(type: kind.as_str(), class: classes!(appearance.class(), class), disabled?: disabled, #attrs) {
            #children
        }
    }
}

component! {
    pub fn LinkButton(href: &str, appearance: Appearance = Appearance::Solid) {
        a(href: href, class: appearance.class(), #attrs) { #children }
    }
}

component! {
    pub fn TextField(
        id: &str,
        name: &str,
        label: &str,
        value: &str = "",
        description: Option<&str> = None,
        error: Option<&str> = None,
        kind: &str = "text",
    ) {
        label("for": id) { (label) }
        input(id: id, name: name, type: kind, value: value, #attrs);
        if let Some(description) = description {
            p(class: "description") { (description) }
        }
        if let Some(error) = error {
            p(class: "error") { (error) }
        }
    }
}

component! {
    pub fn Dialog(
        id: &str,
        title: impl PartialHtml,
        description: Option<&dyn PartialHtml> = None,
        actions: Option<&dyn PartialHtml> = None,
    ) {
        dialog(id: id, aria_labelledby: format!("{id}-title"),
            aria_describedby?: description.as_ref().map(|_| format!("{id}-description")), #attrs) {
            h2(id: format!("{id}-title")) { (title) }
            if let Some(description) = description {
                div(id: format!("{id}-description")) { (description) }
            }
            div(id: format!("{id}-body")) { #children }
            if let Some(actions) = actions {
                footer { (actions) }
            }
        }
    }
}

component! {
    pub fn Legacy<'a, T>(label: &'a str, note: Option<&str>, content: T, count: u32)
    where T: RenderEscaped {
        section(data_count: count, #attrs) {
            (label) (note) (content) #children
        }
    }
}
