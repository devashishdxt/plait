#[path = "../support/default_components.rs"]
mod fixtures;
use fixtures::Dialog;
use plait::html;

fn main() {
    let _ = html! { @Dialog(id: "edit") { "Body" } };
}
