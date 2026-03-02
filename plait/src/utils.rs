use std::fmt;

/// Escapes a HTML string into a writer.
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
