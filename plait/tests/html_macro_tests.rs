use plait::html;

#[test]
fn test_html_macro_text() {
    let html = html! {
        "<div></div>"
    };

    assert_eq!(html.to_string(), "&lt;div&gt;&lt;/div&gt;");
}

#[test]
fn test_html_macro_expr() {
    let text = "Hello World";

    let html = html! {
        (text)
    };

    assert_eq!(html.to_string(), "Hello World");
}

#[test]
fn test_html_macro_text_and_expr() {
    let text = "World";

    let html = html! {
        "Hello " (text)
    };

    assert_eq!(html.to_string(), "Hello World")
}

#[test]
fn test_html_macro_raw_expr() {
    let html = html! {
        #("<div></div>")
    };

    assert_eq!(html.to_string(), "<div></div>")
}

#[test]
fn test_html_macro_doctype() {
    let html = html! {
        #doctype
    };

    assert_eq!(html.to_string(), "<!DOCTYPE html>")
}

#[test]
fn test_html_macro_let_binding() {
    let world = " World";

    let html = html! {
        let hello = world.len();
        (hello) (world)
    };

    assert_eq!(html.to_string(), "6 World")
}

#[test]
fn test_html_macro_if_true() {
    let cond = true;

    let html = html! {
        if cond {
            "Hello World"
        }
    };

    assert_eq!(html.to_string(), "Hello World")
}

#[test]
fn test_html_macro_if_false() {
    let cond = false;

    let html = html! {
        if cond {
            "Hello World"
        }
    };

    assert!(html.to_string().is_empty())
}

#[test]
fn test_html_macro_if_else_true() {
    let cond = true;

    let html = html! {
        if cond {
            "Hello World"
        } else {
            "Goodbye World"
        }
    };

    assert_eq!(html.to_string(), "Hello World")
}

#[test]
fn test_html_macro_if_else_false() {
    let cond = false;

    let html = html! {
        if cond {
            "Hello World"
        } else {
            "Goodbye World"
        }
    };

    assert_eq!(html.to_string(), "Goodbye World")
}

#[test]
fn test_html_macro_if_else_if() {
    let element = "div";

    let html = html! {
        if element == "button" {
            button {}
        } else if element == "div" {
            div {}
        } else {
            "Unknown Element"
        }
    };

    assert_eq!(html.to_string(), "<div></div>")
}

#[test]
fn test_html_macro_if_let_else() {
    let element = Some("div");

    let html = html! {
        if let Some(element) = element {
            "Hello"
            if element == "div" {
                div {}
            } else {
                "Unknown element"
            }
        } else {
            "No Element"
        }
    };

    assert_eq!(html.to_string(), "Hello<div></div>")
}

#[test]
fn test_html_macro_for_loop() {
    let numbers = vec![1, 2, 3];

    let html = html! {
        for number in numbers.iter() {
            li { (number) }
        }
    };

    assert_eq!(html.to_string(), "<li>1</li><li>2</li><li>3</li>")
}

#[test]
fn test_html_macro_match() {
    let element = "div";

    let html = html! {
        match element {
            "button" => {
                "Hello"
                button {}
            },
            "div" => div {},
            _ => "Unknown Element"
        }
    };

    assert_eq!(html.to_string(), "<div></div>")
}

#[test]
fn test_html_macro_element() {
    let html = html! {
        div {
            "Hello "
            span {
                "World"
            }
        }
    };

    assert_eq!(html.to_string(), "<div>Hello <span>World</span></div>")
}

#[test]
fn test_html_macro_void_element() {
    let html = html! {
        div {
            br;
        }
    };

    assert_eq!(html.to_string(), "<div><br></div>")
}

#[test]
fn test_html_macro_custom_element() {
    let html = html! {
        custom_element {
            "Hello "
            span {
                "World"
            }
        }
    };

    assert_eq!(
        html.to_string(),
        "<custom-element>Hello <span>World</span></custom-element>"
    )
}

#[test]
fn test_html_macro_nested() {
    let inner = html! {
        p { "Hello World" }
    };

    let outer = html! {
        div {
            #(&inner)
        }
    };

    assert_eq!(outer.to_string(), "<div><p>Hello World</p></div>");
}

#[test]
fn test_html_macro_attribute_text() {
    let html = html! {
        div(class: "btn") {
            "Hello World"
        }
    };

    assert_eq!(html.to_string(), "<div class=\"btn\">Hello World</div>")
}

#[test]
fn test_html_macro_attribute_str_name() {
    let html = html! {
        div("@click": "callFunction()") {}
    };

    assert_eq!(html.to_string(), "<div @click=\"callFunction()\"></div>")
}

#[test]
fn test_html_macro_attribute_ident_rename() {
    let html = html! {
        div(hx_target: "body") {}
    };

    assert_eq!(html.to_string(), "<div hx-target=\"body\"></div>")
}

#[test]
fn test_html_macro_attribute_raw_expr() {
    let html = html! {
        div(class: #("<div></div>")) {
            "Hello World"
        }
    };

    assert_eq!(
        html.to_string(),
        "<div class=\"<div></div>\">Hello World</div>"
    )
}

#[test]
fn test_html_macro_attribute_without_value() {
    let html = html! {
        button(checked) {
            "Hello World"
        }
    };

    assert_eq!(html.to_string(), "<button checked>Hello World</button>")
}

#[test]
fn test_html_macro_optional_attribute_some() {
    let class = Some("btn");

    let html = html! {
        div(class?: class) {
            "Hello World"
        }
    };

    assert_eq!(html.to_string(), "<div class=\"btn\">Hello World</div>")
}

#[test]
fn test_html_macro_optional_attribute_none() {
    let class = None::<&str>;

    let html = html! {
        div(class?: class) {
            "Hello World"
        }
    };

    assert_eq!(html.to_string(), "<div>Hello World</div>")
}

#[test]
fn test_html_macro_optional_attribute_text() {
    let html = html! {
        div(class?: "btn") {
            "Hello World"
        }
    };

    assert_eq!(html.to_string(), "<div class=\"btn\">Hello World</div>")
}

#[test]
fn test_html_macro_optional_attribute_raw_expr() {
    let class = Some("<div></div>");

    let html = html! {
        div(class?: #(class)) {
            "Hello World"
        }
    };

    assert_eq!(
        html.to_string(),
        "<div class=\"<div></div>\">Hello World</div>"
    )
}

#[test]
fn test_html_macro_boolean_attribute_true() {
    let checked = true;

    let html = html! {
        button(checked?: checked) {
            "Hello World"
        }
    };

    assert_eq!(html.to_string(), "<button checked>Hello World</button>")
}

#[test]
fn test_html_macro_boolean_attribute_false() {
    let checked = false;

    let html = html! {
        button(checked?: checked) {
            "Hello World"
        }
    };

    assert_eq!(html.to_string(), "<button>Hello World</button>")
}

#[test]
fn test_html_macro_multiple_attributes() {
    let class = Some("btn");
    let active = false;

    let html = html! {
        button(id: "button", checked, class?: class, type: "submit", active?: active) {
            "Hello World"
        }
    };

    assert_eq!(
        html.to_string(),
        "<button id=\"button\" checked class=\"btn\" type=\"submit\">Hello World</button>"
    )
}

#[test]
fn test_html_macro_url_attribute_text() {
    let html = html! {
        a(href: "https://example.com") {
            "Hello World"
        }
    };

    assert_eq!(
        html.to_string(),
        "<a href=\"https://example.com\">Hello World</a>"
    )
}

#[test]
fn test_html_macro_url_attribute_expr() {
    let url = "https://example.com";

    let html = html! {
        a(href: url) {
            "Hello World"
        }
    };

    assert_eq!(
        html.to_string(),
        "<a href=\"https://example.com\">Hello World</a>"
    )
}

#[test]
fn test_html_macro_optional_url_attribute_text() {
    let html = html! {
        a(href?: "https://example.com") {
            "Hello World"
        }
    };

    assert_eq!(
        html.to_string(),
        "<a href=\"https://example.com\">Hello World</a>"
    )
}

#[test]
fn test_html_macro_optional_url_attribute_some() {
    let url = Some("https://example.com");

    let html = html! {
        a(href?: url) {
            "Hello World"
        }
    };

    assert_eq!(
        html.to_string(),
        "<a href=\"https://example.com\">Hello World</a>"
    )
}

#[test]
fn test_html_macro_optional_url_attribute_none() {
    let html = html! {
        a(href?: None::<&str>) {
            "Hello World"
        }
    };

    assert_eq!(html.to_string(), "<a>Hello World</a>")
}

#[test]
fn test_html_macro_url_attribute_invalid() {
    let html = html! {
        a(href: "javascript:alert('XSS')") {
            "Hello World"
        }
    };

    assert_eq!(html.to_string(), "<a>Hello World</a>")
}

#[test]
fn test_html_macro_url_attribute_raw_expr() {
    let html = html! {
        a(href: #("javascript:alert('XSS')")) {
            "Hello World"
        }
    };

    assert_eq!(
        html.to_string(),
        "<a href=\"javascript:alert('XSS')\">Hello World</a>"
    )
}
