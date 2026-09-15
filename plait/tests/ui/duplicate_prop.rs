use plait::{component, html};
component! { fn Example(value: &str = "") { (value) } }
fn main() {
    let value = "one";
    let _ = html! { @Example(value, value: "two") {} };
}
