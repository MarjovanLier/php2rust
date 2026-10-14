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
}

impl Helper {
    const ALL: [Helper; 4] = [
        Helper::CtypeDigit,
        Helper::PhpInt,
        Helper::PhpOffset,
        Helper::PhpSubstr,
    ];

    fn name(self) -> &'static str {
        match self {
            Helper::CtypeDigit => "ctype_digit",
            Helper::PhpInt => "php_int",
            Helper::PhpOffset => "php_offset",
            Helper::PhpSubstr => "php_substr",
        }
    }

    /// The function in rustfmt's layout.
    fn text(self) -> &'static str {
        match self {
            Helper::CtypeDigit => CTYPE_DIGIT,
            Helper::PhpInt => PHP_INT,
            Helper::PhpOffset => PHP_OFFSET,
            Helper::PhpSubstr => PHP_SUBSTR,
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
    /// A call of `helper`, which the output then holds.
    pub(super) fn helper_call(&mut self, helper: Helper, arguments: Vec<RExpr>) -> RExpr {
        self.helpers.insert(helper);
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
