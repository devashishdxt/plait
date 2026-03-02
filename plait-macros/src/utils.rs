/// Returns true if the given element name is a void element.
/// Expects the name to be in ASCII lowercase.
pub fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

/// Escapes a HTML string into a writer.
pub fn escape_html_to(writer: &mut String, input: &str) {
    // Fast path for strings without special characters
    if !input
        .bytes()
        .any(|b| matches!(b, b'&' | b'<' | b'>' | b'"' | b'\''))
    {
        writer.push_str(input);
        return;
    }

    let bytes = input.as_bytes();
    let mut last = 0usize;
    let mut i = 0usize;

    while i < bytes.len() {
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

        if last < i {
            writer.push_str(&input[last..i]);
        }
        writer.push_str(replacement);

        i += 1;
        last = i;
    }

    if last < input.len() {
        writer.push_str(&input[last..]);
    }
}
