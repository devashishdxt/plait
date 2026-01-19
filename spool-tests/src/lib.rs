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
    let html = html! {
        div id="myId" class="my-4" checked {
            div id="myI" class="my-4" hx-get=("<script>") {
                "Hello, World!"
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
