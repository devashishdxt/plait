use plait::{component, html};
component! { fn Example(value: &str = "") { (value) } }
fn main() {
    let _ = html! { @Example(value: 42) {} };
}
