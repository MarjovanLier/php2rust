//! PHP names to Rust names (P2R-30, P2R-111).

use heck::{ToSnakeCase, ToUpperCamelCase};

/// Rust 2024 strict and reserved keywords that a raw identifier can carry.
const KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "do", "dyn",
    "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if", "impl", "in", "let",
    "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref", "return",
    "static", "struct", "trait", "true", "try", "type", "typeof", "unsafe", "unsized", "use",
    "virtual", "where", "while", "yield",
];

/// Names that cannot be raw identifiers, and the empty name heck gives for `_`.
const FORBIDDEN: &[&str] = &["self", "super", "crate", ""];

/// The Rust name for a PHP function or variable name (without `$`), or `None` when Rust
/// cannot use it.
pub fn rust_name(php: &str) -> Option<String> {
    let snake = php.to_snake_case();
    if FORBIDDEN.contains(&snake.as_str()) || !ascii_identifier(&snake) {
        return None;
    }
    if KEYWORDS.contains(&snake.as_str()) {
        return Some(format!("r#{snake}"));
    }
    Some(snake)
}

/// The Rust name for a PHP enum or case name, in UpperCamelCase, or `None` when Rust cannot
/// use it: `Self` is the only keyword in that case (P2R-111).
pub fn type_name(php: &str) -> Option<String> {
    let camel = php.to_upper_camel_case();
    (camel != "Self" && ascii_identifier(&camel)).then_some(camel)
}

/// A letter or `_` first, then only ASCII letters, digits and `_`. heck turns `_2` into
/// `2`, and a non-ASCII name trips rustc's confusable-name lints (P2R-30).
fn ascii_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camel_case_becomes_snake_case() {
        assert_eq!(rust_name("getTotal").as_deref(), Some("get_total"));
        assert_eq!(
            rust_name("getHTTPResponse").as_deref(),
            Some("get_http_response")
        );
        assert_eq!(rust_name("a__b").as_deref(), Some("a_b"));
    }

    #[test]
    fn keywords_become_raw_identifiers() {
        assert_eq!(rust_name("type").as_deref(), Some("r#type"));
        assert_eq!(rust_name("gen").as_deref(), Some("r#gen"));
    }

    #[test]
    fn names_rust_cannot_use_are_refused() {
        assert_eq!(rust_name("self"), None);
        assert_eq!(rust_name("Self"), None);
        assert_eq!(rust_name("crate"), None);
        assert_eq!(rust_name("_"), None);
    }

    #[test]
    fn enum_names_become_upper_camel_case() {
        assert_eq!(type_name("Suit").as_deref(), Some("Suit"));
        assert_eq!(type_name("IN_REVIEW").as_deref(), Some("InReview"));
        assert_eq!(type_name("done").as_deref(), Some("Done"));
        assert_eq!(type_name("SELF"), None);
        assert_eq!(type_name("_1"), None);
        assert_eq!(type_name("_"), None);
        assert_eq!(type_name("\u{410}rger"), None);
    }

    #[test]
    fn names_that_are_not_ascii_identifiers_are_refused() {
        assert_eq!(rust_name("_2"), None);
        assert_eq!(rust_name("_3d"), None);
        assert_eq!(rust_name("x\u{b2}"), None);
        assert_eq!(rust_name("\u{430}"), None);
        assert_eq!(rust_name("_x2").as_deref(), Some("x2"));
    }
}
