//! The private helper functions the output holds where PHP's rules differ from Rust's
//! standard library: a cast reads a leading number, `substr` clamps its offsets (P2R-160).

use super::Converter;
use crate::printer::RExpr;

/// A helper, in the order the output holds them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Helper {
    CtypeDigit,
    PhpInt,
    PhpOffset,
    PhpSubstr,
    PhpTrim,
    PhpLtrim,
    PhpRtrim,
    PhpUcfirst,
    PhpLcfirst,
    PhpReplace,
    PhpExplode,
    PhpPregReplace,
    PhpPregMatch,
}

impl Helper {
    const ALL: [Helper; 13] = [
        Helper::CtypeDigit,
        Helper::PhpInt,
        Helper::PhpOffset,
        Helper::PhpSubstr,
        Helper::PhpTrim,
        Helper::PhpLtrim,
        Helper::PhpRtrim,
        Helper::PhpUcfirst,
        Helper::PhpLcfirst,
        Helper::PhpReplace,
        Helper::PhpExplode,
        Helper::PhpPregReplace,
        Helper::PhpPregMatch,
    ];

    fn name(self) -> &'static str {
        match self {
            Helper::CtypeDigit => "ctype_digit",
            Helper::PhpInt => "php_int",
            Helper::PhpOffset => "php_offset",
            Helper::PhpSubstr => "php_substr",
            Helper::PhpTrim => "php_trim",
            Helper::PhpLtrim => "php_ltrim",
            Helper::PhpRtrim => "php_rtrim",
            Helper::PhpUcfirst => "php_ucfirst",
            Helper::PhpLcfirst => "php_lcfirst",
            Helper::PhpReplace => "php_replace",
            Helper::PhpExplode => "php_explode",
            Helper::PhpPregReplace => "php_preg_replace",
            Helper::PhpPregMatch => "php_preg_match",
        }
    }

    /// The function in rustfmt's layout.
    fn text(self) -> &'static str {
        match self {
            Helper::CtypeDigit => CTYPE_DIGIT,
            Helper::PhpInt => PHP_INT,
            Helper::PhpOffset => PHP_OFFSET,
            Helper::PhpSubstr => PHP_SUBSTR,
            Helper::PhpTrim => PHP_TRIM,
            Helper::PhpLtrim => PHP_LTRIM,
            Helper::PhpRtrim => PHP_RTRIM,
            Helper::PhpUcfirst => PHP_UCFIRST,
            Helper::PhpLcfirst => PHP_LCFIRST,
            Helper::PhpReplace => PHP_REPLACE,
            Helper::PhpExplode => PHP_EXPLODE,
            Helper::PhpPregReplace => PHP_PREG_REPLACE,
            Helper::PhpPregMatch => PHP_PREG_MATCH,
        }
    }

    /// The lines of the function, one output item.
    pub(super) fn lines(self) -> Vec<String> {
        self.text().lines().map(str::to_owned).collect()
    }
}

/// True when `name` is the Rust name of a helper, which a function, variable or parameter
/// of that name would hide (P2R-31).
pub(super) fn is_helper(name: &str) -> bool {
    Helper::ALL.iter().any(|helper| helper.name() == name)
}

/// The problem of such a name (P2R-31).
pub(super) const HELPER_NAME: &str =
    "this name is the Rust name of a helper the output may hold, which it would hide (P2R-31)";

impl Converter<'_> {
    /// A call of `helper`, which the module that converts then holds.
    pub(super) fn helper_call(&mut self, helper: Helper, arguments: Vec<RExpr>) -> RExpr {
        self.helpers.insert((self.current_module, helper));
        RExpr::Call(helper.name().to_owned(), arguments)
    }
}

/// `ctype_digit()`: true when the string is not empty and every byte is an ASCII digit.
const CTYPE_DIGIT: &str = r"fn ctype_digit(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit())
}";

/// `(int)` of a string: the number at its start, as PHP reads it (P2R-166).
const PHP_INT: &str = r"fn php_int(text: &str) -> i64 {
    let text = text.trim_start_matches([' ', '\t', '\n', '\r', '\x0B', '\x0C']);
    let bytes = text.as_bytes();
    let mut end = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    let digits = end;
    while bytes.get(end).is_some_and(u8::is_ascii_digit) {
        end += 1;
    }
    let whole = end;
    if bytes.get(end) == Some(&b'.') {
        end += 1;
        while bytes.get(end).is_some_and(u8::is_ascii_digit) {
            end += 1;
        }
    }
    if end == digits || (end == digits + 1 && whole == digits) {
        return 0;
    }
    if matches!(bytes.get(end), Some(b'e' | b'E')) {
        let mut exponent = end + 1;
        if matches!(bytes.get(exponent), Some(b'+' | b'-')) {
            exponent += 1;
        }
        if bytes.get(exponent).is_some_and(u8::is_ascii_digit) {
            end = exponent;
            while bytes.get(end).is_some_and(u8::is_ascii_digit) {
                end += 1;
            }
        }
    }
    if end == whole {
        if let Ok(value) = text[..end].parse::<i64>() {
            return value;
        }
    }
    let value: f64 = text[..end].parse().unwrap_or(0.0);
    if value.is_finite() { value as i64 } else { 0 }
}";

/// `$s[$i]`: the one byte at the offset, which counts from the end when negative
/// (P2R-165).
const PHP_OFFSET: &str = r"fn php_offset(text: &str, index: i64) -> String {
    let at = if index < 0 {
        text.len() as i64 + index
    } else {
        index
    } as usize;
    String::from_utf8_lossy(&text.as_bytes()[at..at + 1]).into_owned()
}";

/// `substr()` with PHP 8's rules for a start or length past either end (P2R-164).
const PHP_SUBSTR: &str = r"fn php_substr(text: &str, start: i64, length: Option<i64>) -> String {
    let size = text.len() as i64;
    if start > size {
        return String::new();
    }
    let start = if start < 0 {
        (size + start).max(0)
    } else {
        start
    };
    let rest = size - start;
    let length = match length {
        Some(length) if length < 0 => (rest + length).max(0),
        Some(length) => length.min(rest),
        None => rest,
    };
    let (from, to) = (start as usize, (start + length) as usize);
    String::from_utf8_lossy(&text.as_bytes()[from..to]).into_owned()
}";

/// `trim()`, `ltrim()` and `rtrim()` with PHP's default bytes, which leave `\x0C` and
/// Unicode spaces where Rust's `trim` removes them (P2R-182).
const PHP_TRIM: &str = r"fn php_trim(text: &str) -> String {
    text.trim_matches([' ', '\t', '\n', '\r', '\0', '\x0B'])
        .to_string()
}";

const PHP_LTRIM: &str = r"fn php_ltrim(text: &str) -> String {
    text.trim_start_matches([' ', '\t', '\n', '\r', '\0', '\x0B'])
        .to_string()
}";

const PHP_RTRIM: &str = r"fn php_rtrim(text: &str) -> String {
    text.trim_end_matches([' ', '\t', '\n', '\r', '\0', '\x0B'])
        .to_string()
}";

/// `ucfirst()` and `lcfirst()`: the first byte when it is an ASCII letter (P2R-183).
const PHP_UCFIRST: &str = r"fn php_ucfirst(text: &str) -> String {
    let mut text = text.to_string();
    if let Some(first) = text.get_mut(..1) {
        first.make_ascii_uppercase();
    }
    text
}";

const PHP_LCFIRST: &str = r"fn php_lcfirst(text: &str) -> String {
    let mut text = text.to_string();
    if let Some(first) = text.get_mut(..1) {
        first.make_ascii_lowercase();
    }
    text
}";

/// `str_replace()`, which gives the subject unchanged for an empty search, where Rust's
/// `replace` inserts the replacement between every character (P2R-185).
const PHP_REPLACE: &str = r"fn php_replace(search: &str, replace: &str, subject: &str) -> String {
    if search.is_empty() {
        subject.to_string()
    } else {
        subject.replace(search, replace)
    }
}";

/// `explode()`, which PHP stops with a `ValueError` on an empty separator (P2R-186).
const PHP_EXPLODE: &str = r#"fn php_explode(separator: &str, text: &str) -> Vec<String> {
    assert!(
        !separator.is_empty(),
        "explode(): separator must not be empty"
    );
    text.split(separator).map(str::to_owned).collect()
}"#;

/// `preg_replace()` of a pattern without `u`, which PCRE matches on bytes with ASCII
/// classes, as `regex::bytes` does in `(?-u)` mode; the replacement holds no group
/// reference (P2R-261).
const PHP_PREG_REPLACE: &str = r#"fn php_preg_replace(pattern: &str, replacement: &str, subject: &str) -> String {
    let regex = ::regex::bytes::Regex::new(pattern).expect("php2rust checked this pattern");
    let replacement = ::regex::bytes::NoExpand(replacement.as_bytes());
    String::from_utf8_lossy(&regex.replace_all(subject.as_bytes(), replacement)).into_owned()
}"#;

/// `preg_match()` of such a pattern where its result is read as a bool (P2R-262).
const PHP_PREG_MATCH: &str = r#"fn php_preg_match(pattern: &str, subject: &str) -> bool {
    ::regex::bytes::Regex::new(pattern)
        .expect("php2rust checked this pattern")
        .is_match(subject.as_bytes())
}"#;
