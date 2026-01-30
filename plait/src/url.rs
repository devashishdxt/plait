/// Returns true if the attribute name is a URL attribute.
pub(crate) fn is_url_attribute(name: &str) -> bool {
    matches!(
        name,
        "href"
            | "src"
            | "action"
            | "formaction"
            | "poster"
            | "cite"
            | "data"
            | "profile"
            | "manifest"
            | "icon"
            | "background"
            | "xlink:href"
    )
}

/// Returns true if the URL is safe to use in HTML. It is important to still escape any special HTML characters after
/// validation.
pub(crate) fn is_url_safe(url: &str) -> bool {
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

/// Returns true if the URL scheme is allowed (i.e., safe).
fn is_allowed_scheme(scheme: &str) -> bool {
    matches!(scheme, "http" | "https" | "mailto" | "tel")
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
    fn test_is_url_safe_http() {
        assert!(is_url_safe("http://example.com"))
    }

    #[test]
    fn test_is_url_safe_http_with_query() {
        assert!(is_url_safe("http://example.com/path?query=1"))
    }

    #[test]
    fn test_is_url_safe_http_with_fragment() {
        assert!(is_url_safe("http://example.com/path#fragment"))
    }

    #[test]
    fn test_is_url_safe_http_with_query_and_fragment() {
        assert!(is_url_safe("http://example.com/path?query=1#fragment"))
    }

    #[test]
    fn test_is_url_safe_https() {
        assert!(is_url_safe("https://example.com"))
    }

    #[test]
    fn test_is_url_safe_https_with_query() {
        assert!(is_url_safe("https://example.com/path?query=1"))
    }

    #[test]
    fn test_is_url_safe_https_with_fragment() {
        assert!(is_url_safe("https://example.com/path#fragment"))
    }

    #[test]
    fn test_is_url_safe_https_with_query_and_fragment() {
        assert!(is_url_safe("https://example.com/path?query=1#fragment"))
    }

    #[test]
    fn test_is_url_safe_mailto() {
        assert!(is_url_safe("mailto:test@example.com"))
    }

    #[test]
    fn test_is_url_safe_tel() {
        assert!(is_url_safe("tel:+1234567890"))
    }

    #[test]
    fn test_is_url_safe_relative_path() {
        assert!(is_url_safe("path/to/page"))
    }

    #[test]
    fn test_is_url_safe_relative_slash() {
        assert!(is_url_safe("/path/to/page"))
    }

    #[test]
    fn test_is_url_safe_relative_dot() {
        assert!(is_url_safe("./relative/path"))
    }

    #[test]
    fn test_is_url_safe_relative_dot_dot() {
        assert!(is_url_safe("../relative/path"))
    }

    #[test]
    fn test_is_url_safe_relative_query() {
        assert!(is_url_safe("?query=value"))
    }

    #[test]
    fn test_is_url_safe_relative_fragment() {
        assert!(is_url_safe("#section"))
    }

    #[test]
    fn test_is_url_safe_with_html_special_chars() {
        assert!(is_url_safe("https://example.com/path?a=1&b=\"2\""))
    }

    #[test]
    fn test_is_url_safe_empty_blocked() {
        assert!(!is_url_safe(""))
    }

    #[test]
    fn test_is_url_safe_javascript_blocked() {
        assert!(!is_url_safe("javascript:alert(1)"))
    }

    #[test]
    fn test_is_url_safe_javascript_with_whitespace_blocked() {
        assert!(!is_url_safe("  javascript:alert(1)"))
    }

    #[test]
    fn test_is_url_safe_javascript_with_inner_whitespace_blocked() {
        assert!(!is_url_safe("javascript  :alert(1)"))
    }

    #[test]
    fn test_is_url_safe_javascript_with_control_chars_blocked() {
        assert!(!is_url_safe("java\x00script:alert(1)"))
    }

    #[test]
    fn test_is_url_safe_javascript_with_newline_blocked() {
        assert!(!is_url_safe("java\nscript:alert(1)"))
    }

    #[test]
    fn test_is_url_safe_javascript_with_tab_blocked() {
        assert!(!is_url_safe("java\tscript:alert(1)"))
    }

    #[test]
    fn test_is_url_safe_javascript_mixed_case_blocked() {
        assert!(!is_url_safe("JaVaScRiPt:alert(1)"))
    }

    #[test]
    fn test_is_url_safe_javascript_upper_case_blocked() {
        assert!(!is_url_safe("JAVASCRIPT:alert(1)"))
    }

    #[test]
    fn test_is_url_safe_javascript_encoded_blocked() {
        assert!(!is_url_safe("javascript%3Aalert(1)"))
    }

    #[test]
    fn test_is_url_safe_javascript_html_entity_blocked() {
        assert!(!is_url_safe("java&#115;cript:alert(1)"))
    }

    #[test]
    fn test_is_url_safe_javascript_double_percent_encoded_blocked() {
        assert!(!is_url_safe(
            "%256a%2561%2576%2561%2573%2563%2572%2569%2570%2574%253aalert(1)"
        ))
    }

    #[test]
    fn test_is_url_safe_data_blocked() {
        assert!(!is_url_safe("data:text/String,<script>alert(1)</script>"))
    }

    #[test]
    fn test_is_url_safe_vbscript_blocked() {
        assert!(!is_url_safe("vbscript:msgbox(1)"))
    }

    #[test]
    fn test_is_url_safe_file_blocked() {
        assert!(!is_url_safe("file:///etc/passwd"))
    }

    #[test]
    fn test_is_url_safe_blob_blocked() {
        assert!(!is_url_safe(
            "blob:https://example.com/550e8400-e29b-41d4-a716-446655440000"
        ))
    }

    #[test]
    fn test_is_url_safe_about_blocked() {
        assert!(!is_url_safe("about:blank"))
    }

    #[test]
    fn test_is_url_safe_ws_blocked() {
        assert!(!is_url_safe("ws://example.com/socket"))
    }

    #[test]
    fn test_is_url_safe_wss_blocked() {
        assert!(!is_url_safe("wss://example.com/socket"))
    }

    #[test]
    fn test_is_url_safe_ftp_blocked() {
        assert!(!is_url_safe("ftp://example.com/file"))
    }
}
