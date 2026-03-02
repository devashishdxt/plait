use std::fmt;

/// Escapes HTML-special characters in `input` and writes the result into `writer`.
///
/// The following characters are replaced:
///
/// | Character | Replacement |
/// |-----------|-------------|
/// | `&`       | `&amp;`     |
/// | `<`       | `&lt;`      |
/// | `>`       | `&gt;`      |
/// | `"`       | `&quot;`    |
/// | `'`       | `&#39;`     |
///
/// Characters that don't need escaping are written through in bulk for performance.
pub fn escape_html_to(writer: &mut (impl fmt::Write + ?Sized), input: &str) -> fmt::Result {
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
            writer.write_str(&input[last..i])?;
        }
        writer.write_str(replacement)?;

        i += 1;
        last = i;
    }

    if last < input.len() {
        writer.write_str(&input[last..])?;
    }

    Ok(())
}
