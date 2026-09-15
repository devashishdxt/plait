#[path = "../support/default_components.rs"]
mod fixtures;
use fixtures::LinkButton;
use plait::html;

fn main() {
    let _ = html! { @LinkButton { "Home" } };
}
