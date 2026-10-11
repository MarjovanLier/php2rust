//! Rust text: string literals, format strings and line width (P2R-45, P2R-49, P2R-51).

/// rustfmt's `max_width`: a longer line is split, so php2rust refuses it (P2R-51).
pub const MAX_WIDTH: usize = 100;

/// A Rust string literal holding `text`.
pub fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\0' => out.push_str("\\0"),
            c if c.is_control() => out.push_str(&format!("\\u{{{:x}}}", u32::from(c))),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// A Rust format string that prints `text` unchanged: braces are doubled (P2R-45).
pub fn format_text(text: &str) -> String {
    quote(&text.replace('{', "{{").replace('}', "}}"))
}

/// True when a double-quoted PHP string holds an octal escape above `\377`, which PHP
/// silently truncates to one byte and Mago decodes the same way (P2R-49).
pub fn octal_overflow(raw: &[u8]) -> bool {
    let mut i = 0;
    while i < raw.len() {
        if raw[i] != b'\\' {
            i += 1;
            continue;
        }
        let digits: Vec<u8> = raw[i + 1..]
            .iter()
            .take(3)
            .take_while(|b| (b'0'..=b'7').contains(b))
            .copied()
            .collect();
        if digits.is_empty() {
            // Skips the escaped character too, so `\\101` is a backslash and then text.
            i += 2;
            continue;
        }
        if digits
            .iter()
            .fold(0_u32, |value, d| value * 8 + u32::from(d - b'0'))
            > 0o377
        {
            return true;
        }
        i += 1 + digits.len();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quote_escapes_what_rust_needs() {
        assert_eq!(
            quote("a\"b\\c\nd\te\u{1b}"),
            "\"a\\\"b\\\\c\\nd\\te\\u{1b}\""
        );
    }

    #[test]
    fn format_text_doubles_braces() {
        assert_eq!(format_text("{x}"), "\"{{x}}\"");
    }

    #[test]
    fn octal_overflow_finds_only_values_above_377() {
        assert!(octal_overflow(br#""\400""#));
        assert!(!octal_overflow(br#""\377""#));
        assert!(!octal_overflow(br#""\\400""#));
        assert!(!octal_overflow(br#""\x41""#));
    }
}
