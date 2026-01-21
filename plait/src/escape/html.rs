use crate::Html;

/// Escapes HTML special characters in a string and writes the result to the output HTML.
pub fn escape_html(output: &mut Html, s: &str) {
    hescape::escape_to(output, s).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_html_basic() {
        let mut output = Html::new();
        escape_html(&mut output, "Hello, World!");
        assert_eq!(output, "Hello, World!");
    }

    #[test]
    fn escape_html_quotes() {
        let mut output = Html::new();
        escape_html(&mut output, r#"He said "hello""#);
        assert_eq!(output, "He said &quot;hello&quot;");
    }

    #[test]
    fn escape_html_single_quotes() {
        let mut output = Html::new();
        escape_html(&mut output, "It's fine");
        assert_eq!(output, "It&#39;s fine");
    }

    #[test]
    fn escape_html_all_special() {
        let mut output = Html::new();
        escape_html(&mut output, "<\"'>&");
        assert_eq!(output, "&lt;&quot;&#39;&gt;&amp;");
    }

    #[test]
    fn escape_unicode() {
        let mut output = Html::new();
        escape_html(&mut output, "Hello <世界>");
        assert_eq!(output, "Hello &lt;世界&gt;");
    }

    #[test]
    fn escape_empty_string() {
        let mut output = Html::new();
        escape_html(&mut output, "");
        assert_eq!(output, "");
    }
}
