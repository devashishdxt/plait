use crate::Html;

/// Escapes HTML special characters in a string and writes the result to the output HTML.
pub fn escape_html(output: &mut Html, s: &str) {
    hescape::escape_to(output.inner_mut(), s).unwrap()
}
