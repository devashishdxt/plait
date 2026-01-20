use spool::{Render, attrs, html};

pub fn attrs() {
    let attrs = attrs!(
        id = "myId"
        class = "my-4"
        checked
        ..(
            attrs!(
                id = "myI"
                class = "my-4"
                hx-get = ("<script>")
            )
        )
    );

    println!("{}", attrs.render(spool::EscapeMode::Raw).0);
}

pub fn html() {
    let num = Some(5.01);
    let a = "hello1";

    let html = html! {
        div id="myId" class="my-4" checked ..(attrs!(id="my" class="mx-4")) {
            div id="myI" class="my-4" hx-get=("<script>") {
                "Hello, World!"
            }
            @if let Some(n) = num {
                div id="myII" class="my-4" hx-get=("<script>") {
                    (n)
                }
            } @else {
                div id="myV" class="my-4" hx-get=("<script>") {
                    "Hello, World!"
                }
            }
            @for i in 1..=5 {
                div class="my-4" hx-get=("<script>") {
                    (i)
                }
            }
            @match a {
                "hello" => "Hello world!",
                _ => "Goodbye, World!",
            }
            br class="my-4";
        }
    };

    println!("{}", html);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attrs() {
        attrs();
    }

    #[test]
    fn test_html() {
        html();
    }
}
