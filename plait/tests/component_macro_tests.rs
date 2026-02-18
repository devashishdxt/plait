use plait::{HtmlDisplay, classes, component, html};

component! {
    pub fn Button<'a>(class: Option<&'a str>) {
        button(class: classes!("btn", class), #attrs) {
            #children
        }
    }
}

component! {
    pub fn Card<T>(title: T) where T: HtmlDisplay {
        div(class: "card") {
            h1 { @(title) }
            @Button(class: "btn-primary".into(); #attrs) {
                #children
            }
        }
    }
}

#[test]
fn test_button() {
    let disabled = false;

    let html = html! {
        @Button(class: None; id: "btn1", disabled?: disabled) {
            "Click me"
        }
    };

    assert_eq!(
        html.to_string(),
        "<button class=\"btn\" id=\"btn1\">Click me</button>"
    );
}

#[test]
fn test_card() {
    let disabled = true;

    let html = html! {
        @Card(title: html! { span { "My card" } }; disabled?: disabled) {
            "Click me"
        }
    };

    assert_eq!(
        html.to_string(),
        "<div class=\"card\"><h1><span>My card</span></h1><button class=\"btn btn-primary\" disabled>Click me</button></div>"
    );
}
