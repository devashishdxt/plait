use plait::{component, html, ToHtml};
component! { fn Example(value: &str = "") { (value) } }
fn main() {
    let page;
    {
        let owner = String::from("value");
        let value = owner.as_str();
        page = html! { @Example(value) {} };
    }
    let _ = page.to_html();
}
