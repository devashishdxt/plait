use defaults_components::{Action, Dialog, TextField};
use plait::{ToHtml, html};

fn main() {
    let value = String::from("borrowed");
    let error = Some(String::from("Invalid"));
    let (value, error) = (value.as_str(), error.as_deref());
    let actions = html! { @Action { "Save" } };
    let actions = &actions;
    let minimal = html! { @Dialog(id: "minimal", title: html! { "Title" }) {} }.to_html();
    assert!(!minimal.contains("<footer>"));
    let page = html! {
        @defaults_components::Action(class: "layout"; data_test: "qualified") { "Open" }
        @Dialog(id: "edit", title: html! { "Edit" }, actions: Some(actions)) {
            @TextField(id: "name", name: "name", label: "Name", value, error; hx_post: "/check") {}
        }
    }
    .to_html();
    assert!(page.contains("class=\"solid layout\" data-test=\"qualified\""));
    assert!(page.contains("value=\"borrowed\" hx-post=\"/check\""));
    assert!(page.contains("<p class=\"error\">Invalid</p>"));
    assert!(
        page.contains("<footer><button type=\"button\" class=\"solid\">Save</button></footer>")
    );
}
