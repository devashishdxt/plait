use plait::{ClassPart, classes, component, html};

#[test]
fn test_classes_macro() {
    let html = html! {
        button(class: classes!("btn", "btn-primary")) {}
    };

    assert_eq!(
        html.to_string(),
        "<button class=\"btn btn-primary\"></button>"
    )
}

#[test]
fn test_classes_macro_in_component() {
    component! {
        fn Button<'a>(class: Option<&'a str>) {
            button(class: classes!("btn", class)) {}
        }
    }

    let button_none = html! {
        @Button(class: None) {}
    };

    assert_eq!(button_none.to_string(), "<button class=\"btn\"></button>");

    let button_some = html! {
        @Button(class: Some("btn-primary")) {}
    };

    assert_eq!(
        button_some.to_string(),
        "<button class=\"btn btn-primary\"></button>"
    );
}

#[test]
fn test_classes_macro_in_component_with_class_part() {
    component! {
        fn Button<C>(class: C) where C: ClassPart {
            button(class: classes!("btn", class)) {}
        }
    }

    let button_none = html! {
        @Button(class: None::<&str>) {}
    };

    assert_eq!(button_none.to_string(), "<button class=\"btn\"></button>");

    let button_some = html! {
        @Button(class: Some("btn-primary")) {}
    };

    assert_eq!(
        button_some.to_string(),
        "<button class=\"btn btn-primary\"></button>"
    );

    let button_classes = html! {
        @Button(class: classes!("btn-secondary", "btn-lg")) {}
    };

    assert_eq!(
        button_classes.to_string(),
        "<button class=\"btn btn-secondary btn-lg\"></button>"
    );
}
