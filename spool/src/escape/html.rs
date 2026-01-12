use crate::Error;

pub fn escape_html(output: &mut String, s: &str) -> Result<(), Error> {
    // Fast path: if there are no bytes that need escaping, write once and return.
    if !needs_html_escape(s) {
        output.push_str(s);
        return Ok(());
    }

    let bytes = s.as_bytes();
    let mut last = 0usize;
    let mut i = 0usize;

    while i < bytes.len() {
        // Match only ASCII bytes. In valid UTF-8, bytes < 0x80 are single-byte chars.
        let replacement = match bytes[i] {
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'"' => "&quot;",
            b'\'' => "&#39;",
            _ => {
                i += 1;
                continue;
            }
        };

        // `i` is guaranteed to be on a UTF-8 boundary here: these ASCII bytes cannot occur inside a multibyte UTF-8
        // sequence.
        if last < i {
            output.push_str(&s[last..i]);
        }
        output.push_str(replacement);

        i += 1; // consumed the ASCII byte
        last = i; // next chunk starts after it
    }

    if last < s.len() {
        output.push_str(&s[last..]);
    }

    Ok(())
}

#[inline]
fn needs_html_escape(s: &str) -> bool {
    // Byte scan is safe: we only care about ASCII bytes.
    s.bytes()
        .any(|b| matches!(b, b'&' | b'<' | b'>' | b'"' | b'\''))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_html_basic() {
        let mut output = String::new();
        escape_html(&mut output, "Hello, World!").unwrap();
        assert_eq!(output, "Hello, World!");
    }

    #[test]
    fn escape_html_quotes() {
        let mut output = String::new();
        escape_html(&mut output, r#"He said "hello""#).unwrap();
        assert_eq!(output, "He said &quot;hello&quot;");
    }

    #[test]
    fn escape_html_single_quotes() {
        let mut output = String::new();
        escape_html(&mut output, "It's fine").unwrap();
        assert_eq!(output, "It&#39;s fine");
    }

    #[test]
    fn escape_html_all_special() {
        let mut output = String::new();
        escape_html(&mut output, "<\"'>&").unwrap();
        assert_eq!(output, "&lt;&quot;&#39;&gt;&amp;");
    }

    #[test]
    fn needs_html_escape_detection() {
        assert!(!needs_html_escape("Hello, World!"));
        assert!(needs_html_escape("<script>"));
        assert!(needs_html_escape("\"quoted\""));
        assert!(needs_html_escape("foo & bar"));
        assert!(needs_html_escape("a > b"));
        assert!(needs_html_escape("It's fine"));
    }

    #[test]
    fn escape_unicode() {
        let mut output = String::new();
        escape_html(&mut output, "Hello <世界>").unwrap();
        assert_eq!(output, "Hello &lt;世界&gt;");
    }

    #[test]
    fn escape_empty_string() {
        let mut output = String::new();
        escape_html(&mut output, "").unwrap();
        assert_eq!(output, "");
    }
}
