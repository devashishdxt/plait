use spool::{Render, attrs};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attrs() {
        attrs();
    }
}
