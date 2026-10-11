//! PHP names to Rust names (P2R-30).

use heck::ToSnakeCase;

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
    if FORBIDDEN.contains(&snake.as_str()) {
        return None;
    }
    if KEYWORDS.contains(&snake.as_str()) {
        return Some(format!("r#{snake}"));
    }
    Some(snake)
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
}
