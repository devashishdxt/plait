use plait::{ClassPart, classes, component, html, render};

#[test]
fn test_classes_macro() {
    let html = render(html! {
        button(class: classes!("btn", "btn-primary")) {}
    });

    assert_eq!(html, "<button class=\"btn btn-primary\"></button>")
}

#[test]
fn test_classes_macro_in_component() {
    component! {
        fn Button<'a>(class: Option<&'a str>) {
            button(class: classes!("btn", class)) {}
        }
    }

    let button_none = render(html! {
        @Button(class: None) {}
    });

    assert_eq!(button_none, "<button class=\"btn\"></button>");

    let button_some = render(html! {
        @Button(class: Some("btn-primary")) {}
    });

    assert_eq!(button_some, "<button class=\"btn btn-primary\"></button>");
}

#[test]
fn test_classes_macro_in_component_with_class_part() {
    component! {
        fn Button<C>(class: C) where C: ClassPart {
            button(class: classes!("btn", class)) {}
        }
    }

    let button_none = render(html! {
        @Button(class: None) {}
    });

    assert_eq!(button_none, "<button class=\"btn\"></button>");

    let button_some = render(html! {
        @Button(class: Some("btn-primary")) {}
    });

    assert_eq!(button_some, "<button class=\"btn btn-primary\"></button>");

    let button_classes = render(html! {
        @Button(class: classes!("btn-secondary", "btn-lg")) {}
    });

    assert_eq!(
        button_classes,
        "<button class=\"btn btn-secondary btn-lg\"></button>"
    );
}
