#[path = "../support/default_components.rs"]
mod fixtures;
use fixtures::Legacy;
use plait::html;

fn main() {
    let _ = html! { @Legacy(label: "Label", note: None, content: html! { "Content" }) {} };
}
