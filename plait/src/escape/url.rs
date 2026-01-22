use crate::Html;

/// Safe fallback URL used when a potentially dangerous URL is blocked.
const BLOCKED_URL_FALLBACK: &str = "about:invalid";

fn is_allowed_scheme(scheme: &str) -> bool {
    matches!(scheme, "http" | "https" | "mailto" | "tel")
}

/// Escapes URL special characters in a string and writes the result to the output HTML.
pub fn escape_url(output: &mut Html, s: &str) {
    if !is_safe_url(s) {
        output.0.push_str(BLOCKED_URL_FALLBACK);
    } else {
        hescape::escape_to(output, s).unwrap();
    }
}

fn is_safe_url(url: &str) -> bool {
    if url.is_empty() {
        return false;
    }

    // Double percent decoding based on OWASP guidelines
    let decoded = match percent_encoding::percent_decode_str(url).decode_utf8() {
        Ok(decoded) => decoded,
        Err(_) => return false,
    };
    let decoded = match percent_encoding::percent_decode_str(&decoded).decode_utf8() {
        Ok(decoded) => decoded,
        Err(_) => return false,
    };
    let decoded = hescape::unescape(&decoded);
    let decoded: String = decoded.chars().filter(|c| !c.is_control()).collect();

    let trimmed = decoded.trim();

    if trimmed.is_empty() {
        return false;
    }

    let scheme = match parse_scheme(trimmed) {
        Some(scheme) => scheme,
        None => return true,
    };

    if !is_allowed_scheme(&scheme) {
        return false;
    }

    true
}

/// Parses a scheme allowing ASCII whitespace/control between token and ':'.
/// Recognizes `javascript    :alert(1)` as `javascript`.
fn parse_scheme(s: &str) -> Option<String> {
    let colon = s.find(':')?;

    // If '/', '?', or '#' appears before ':', it is relative
    if let Some(i) = s.find(['/', '?', '#'])
        && i < colon
    {
        return None;
    }

    let prefix = &s[..colon];
    if prefix.is_empty() {
        return None;
    }

    let mut chars = prefix.char_indices();

    // First char must be ASCII alphabetic
    let (_, first) = chars.next()?;
    if !first.is_ascii_alphabetic() {
        return None;
    }

    // Parse scheme token
    let mut token_end = first.len_utf8();
    for (i, ch) in chars {
        if ch.is_ascii_alphanumeric() || ch == '+' || ch == '-' || ch == '.' {
            token_end = i + ch.len_utf8();
        } else {
            break;
        }
    }

    // Between token end and colon: only ASCII whitespace/control
    if !prefix[token_end..]
        .chars()
        .all(|c| c.is_ascii_whitespace() || c.is_ascii_control())
    {
        return None;
    }

    Some(prefix[..token_end].to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_url_http() {
        let mut output = Html::new();
        escape_url(&mut output, "http://example.com");
        assert_eq!(output, "http://example.com");
    }

    #[test]
    fn escape_url_https() {
        let mut output = Html::new();
        escape_url(&mut output, "https://example.com/path?query=1");
        assert_eq!(output, "https://example.com/path?query=1");
    }

    #[test]
    fn escape_url_mailto() {
        let mut output = Html::new();
        escape_url(&mut output, "mailto:test@example.com");
        assert_eq!(output, "mailto:test@example.com");
    }

    #[test]
    fn escape_url_tel() {
        let mut output = Html::new();
        escape_url(&mut output, "tel:+1234567890");
        assert_eq!(output, "tel:+1234567890");
    }

    #[test]
    fn escape_url_relative_slash() {
        let mut output = Html::new();
        escape_url(&mut output, "/path/to/page");
        assert_eq!(output, "/path/to/page");
    }

    #[test]
    fn escape_url_relative_dot() {
        let mut output = Html::new();
        escape_url(&mut output, "./relative/path");
        assert_eq!(output, "./relative/path");
    }

    #[test]
    fn escape_url_relative_dotdot() {
        let mut output = Html::new();
        escape_url(&mut output, "../parent/path");
        assert_eq!(output, "../parent/path");
    }

    #[test]
    fn escape_url_relative_query() {
        let mut output = Html::new();
        escape_url(&mut output, "?query=value");
        assert_eq!(output, "?query=value");
    }

    #[test]
    fn escape_url_relative_fragment() {
        let mut output = Html::new();
        escape_url(&mut output, "#section");
        assert_eq!(output, "#section");
    }

    #[test]
    fn escape_url_empty() {
        let mut output = Html::new();
        escape_url(&mut output, "");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_javascript_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "javascript:alert(1)");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_data_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "data:text/html,<script>alert(1)</script>");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_javascript_encoded_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "javascript%3Aalert(1)");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_javascript_html_entity_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "java&#115;cript:alert(1)");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_with_html_special_chars() {
        let mut output = Html::new();
        escape_url(&mut output, "https://example.com/path?a=1&b=2");
        assert_eq!(output, "https://example.com/path?a=1&amp;b=2");
    }

    #[test]
    fn escape_url_with_quotes() {
        let mut output = Html::new();
        escape_url(&mut output, "https://example.com/path?q=\"test\"");
        assert_eq!(output, "https://example.com/path?q=&quot;test&quot;");
    }

    #[test]
    fn escape_url_with_angle_brackets() {
        let mut output = Html::new();
        escape_url(&mut output, "https://example.com/<path>");
        assert_eq!(output, "https://example.com/&lt;path&gt;");
    }

    #[test]
    fn escape_url_vbscript_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "vbscript:msgbox(1)");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_file_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "file:///etc/passwd");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_javascript_with_whitespace_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "  javascript:alert(1)");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_javascript_with_control_chars_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "java\x00script:alert(1)");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_javascript_with_inner_whitespace_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "javascript    :alert(1)");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_javascript_with_newline_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "java\nscript:alert(1)");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_javascript_with_tab_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "java\tscript:alert(1)");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_javascript_mixed_case_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "JaVaScRiPt:alert(1)");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_javascript_uppercase_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "JAVASCRIPT:alert(1)");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    // Edge case tests for potential gaps

    #[test]
    fn escape_url_double_percent_encoded_javascript_blocked() {
        // %6a = 'j', so %256a decodes to %6a which decodes to 'j'
        // This is double-encoded "javascript:alert(1)"
        let mut output = Html::new();
        escape_url(
            &mut output,
            "%256a%2561%2576%2561%2573%2563%2572%2569%2570%2574%253aalert(1)",
        );
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_blob_blocked() {
        let mut output = Html::new();
        escape_url(
            &mut output,
            "blob:https://example.com/550e8400-e29b-41d4-a716-446655440000",
        );
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_about_blank_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "about:blank");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_unicode_fullwidth_javascript() {
        // Fullwidth characters: ｊａｖａｓｃｒｉｐｔ：
        // This is treated as relative URL by browser
        let mut output = Html::new();
        escape_url(&mut output, "ｊａｖａｓｃｒｉｐｔ:alert(1)");
        assert_eq!(output, "ｊａｖａｓｃｒｉｐｔ:alert(1)");
    }

    #[test]
    fn escape_url_bare_hostname_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "example.com");
        assert_eq!(output, "example.com");
    }

    #[test]
    fn escape_url_bare_hostname_with_path_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "example.com/path");
        assert_eq!(output, "example.com/path");
    }

    #[test]
    fn escape_url_very_long_url() {
        let mut output = Html::new();
        let long_path = "a".repeat(10000);
        let url = format!("https://example.com/{}", long_path);
        escape_url(&mut output, &url);
        assert!(output.len() > 10000);
    }

    #[test]
    fn escape_url_ws_websocket_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "ws://example.com/socket");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_wss_websocket_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "wss://example.com/socket");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }

    #[test]
    fn escape_url_ftp_blocked() {
        let mut output = Html::new();
        escape_url(&mut output, "ftp://example.com/file");
        assert_eq!(output, BLOCKED_URL_FALLBACK);
    }
}
