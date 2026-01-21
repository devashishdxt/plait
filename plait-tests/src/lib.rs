use plait::{Render, attrs, html, EscapeMode};

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

    println!("{}", attrs.render(EscapeMode::Raw).0);
}

pub fn html() {
    let num = Some(4.9);
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
            @match num {
                Some(n) if n > 5.0 => "Number is greater than 5",
                Some(_) => "Number is less than or equal to 5",
                None => "Number is None",
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
