use plait::{Class, ToHtml, classes, component, html};

#[test]
fn test_classes_macro() {
    let html = html! {
        button(class: &classes!("btn", "btn-primary")) {}
    };

    assert_eq!(
        html.to_html(),
        "<button class=\"btn btn-primary\"></button>"
    )
}

#[test]
fn test_classes_macro_in_component() {
    component! {
        fn Button<'a>(class: Option<&'a str>) {
            button(class: &classes!("btn", class)) {}
        }
    }

    let button_none = html! {
        @Button(class: None) {}
    };

    assert_eq!(button_none.to_html(), "<button class=\"btn\"></button>");

    let button_some = html! {
        @Button(class: Some("btn-primary")) {}
    };

    assert_eq!(
        button_some.to_html(),
        "<button class=\"btn btn-primary\"></button>"
    );
}

#[test]
fn test_classes_macro_in_component_with_class_part() {
    component! {
        fn Button(class: impl Class) {
            button(class: &classes!("btn", class)) {}
        }
    }

    let button_none = html! {
        @Button(class: None::<&str>) {}
    };

    assert_eq!(button_none.to_html(), "<button class=\"btn\"></button>");

    let button_some = html! {
        @Button(class: Some("btn-primary")) {}
    };

    assert_eq!(
        button_some.to_html(),
        "<button class=\"btn btn-primary\"></button>"
    );

    let button_classes = html! {
        @Button(class: classes!("btn-secondary", "btn-lg")) {}
    };

    assert_eq!(
        button_classes.to_html(),
        "<button class=\"btn btn-secondary btn-lg\"></button>"
    );
}
