#[path = "../support/default_components.rs"]
mod fixtures;
use fixtures::TextField;
use plait::html;

fn main() {
    let _ = html! { @TextField(name: "email", label: "Email") {} };
}
