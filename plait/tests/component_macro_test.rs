use plait::{component, html, render, render_with_capacity};

component! {
    pub fn Button<'a>(class: &'a str) {
        button(class: format_args!("btn {class}"), #attrs) {
            #children
        }
    }
}

component! {
    pub fn Card {
        div(class: "card") {
            @Button(class: "btn-primary"; #attrs) {
                #children
            }
        }
    }
}

#[test]
fn test_button() {
    let disabled = false;

    let html = render(html! {
        @Button(class: "btn-primary"; id: "btn1", disabled?: disabled) {
            "Click me"
        }
    });

    assert_eq!(
        html,
        "<button class=\"btn btn-primary\" id=\"btn1\">Click me</button>"
    );
}

#[test]
fn test_card() {
    let disabled = true;

    let html = render_with_capacity(
        1024,
        html! {
            @Card(; disabled?: disabled) {
                "Click me"
            }
        },
    );

    assert_eq!(
        html,
        "<div class=\"card\"><button class=\"btn btn-primary\" disabled>Click me</button></div>"
    );
}
