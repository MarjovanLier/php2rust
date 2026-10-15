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
    PhpStrtr,
    PhpCharmask,
    PhpTrimChars,
    PhpLtrimChars,
    PhpRtrimChars,
    PhpCheckdate,
    PhpUcwords,
    PhpPregReplaceCallback,
}

impl Helper {
    const ALL: [Helper; 21] = [
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
        Helper::PhpStrtr,
        Helper::PhpCharmask,
        Helper::PhpTrimChars,
        Helper::PhpLtrimChars,
        Helper::PhpRtrimChars,
        Helper::PhpCheckdate,
        Helper::PhpUcwords,
        Helper::PhpPregReplaceCallback,
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
            Helper::PhpStrtr => "php_strtr",
            Helper::PhpCharmask => "php_charmask",
            Helper::PhpTrimChars => "php_trim_chars",
            Helper::PhpLtrimChars => "php_ltrim_chars",
            Helper::PhpRtrimChars => "php_rtrim_chars",
            Helper::PhpCheckdate => "php_checkdate",
            Helper::PhpUcwords => "php_ucwords",
            Helper::PhpPregReplaceCallback => "php_preg_replace_callback",
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
            Helper::PhpStrtr => PHP_STRTR,
            Helper::PhpCharmask => PHP_CHARMASK,
            Helper::PhpTrimChars => PHP_TRIM_CHARS,
            Helper::PhpLtrimChars => PHP_LTRIM_CHARS,
            Helper::PhpRtrimChars => PHP_RTRIM_CHARS,
            Helper::PhpCheckdate => PHP_CHECKDATE,
            Helper::PhpUcwords => PHP_UCWORDS,
            Helper::PhpPregReplaceCallback => PHP_PREG_REPLACE_CALLBACK,
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

/// `strtr()` with a map: at each position the longest key that matches, whatever the order
/// of the pairs, and no second look at a value it put in. Every key is valid UTF-8, so a
/// step by characters where no key matches gives PHP's byte result (P2R-282).
const PHP_STRTR: &str = r"fn php_strtr(text: &str, pairs: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(next) = rest.chars().next() {
        let longest = pairs
            .iter()
            .filter(|(key, _)| !key.is_empty() && rest.starts_with(key))
            .max_by_key(|(key, _)| key.len());
        match longest {
            Some((key, value)) => {
                out.push_str(value);
                rest = &rest[key.len()..];
            }
            None => {
                out.push(next);
                rest = &rest[next.len_utf8()..];
            }
        }
    }
    out
}";

/// PHP's byte mask of a `trim()` character list: `a..z` adds a range, and a `..` that
/// starts no range adds nothing, where PHP warns (P2R-291).
const PHP_CHARMASK: &str = r"fn php_charmask(characters: &str) -> [bool; 256] {
    let input = characters.as_bytes();
    let mut mask = [false; 256];
    let mut at = 0;
    while at < input.len() {
        let first = input[at];
        if at + 3 < input.len()
            && input[at + 1] == b'.'
            && input[at + 2] == b'.'
            && input[at + 3] >= first
        {
            for byte in first..=input[at + 3] {
                mask[usize::from(byte)] = true;
            }
            at += 4;
        } else if at + 1 < input.len() && first == b'.' && input[at + 1] == b'.' {
            at += 1;
        } else {
            mask[usize::from(first)] = true;
            at += 1;
        }
    }
    mask
}";

/// `trim()`, `ltrim()` and `rtrim()` with a character list, on bytes (P2R-291).
const PHP_TRIM_CHARS: &str = r"fn php_trim_chars(text: &str, characters: &str) -> String {
    let mask = php_charmask(characters);
    let bytes = text.as_bytes();
    let start = bytes
        .iter()
        .position(|&byte| !mask[usize::from(byte)])
        .unwrap_or(bytes.len());
    let end = bytes
        .iter()
        .rposition(|&byte| !mask[usize::from(byte)])
        .map_or(start, |last| last + 1);
    String::from_utf8_lossy(&bytes[start..end]).into_owned()
}";

const PHP_LTRIM_CHARS: &str = r"fn php_ltrim_chars(text: &str, characters: &str) -> String {
    let mask = php_charmask(characters);
    let bytes = text.as_bytes();
    let start = bytes
        .iter()
        .position(|&byte| !mask[usize::from(byte)])
        .unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[start..]).into_owned()
}";

const PHP_RTRIM_CHARS: &str = r"fn php_rtrim_chars(text: &str, characters: &str) -> String {
    let mask = php_charmask(characters);
    let bytes = text.as_bytes();
    let end = bytes
        .iter()
        .rposition(|&byte| !mask[usize::from(byte)])
        .map_or(0, |last| last + 1);
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}";

/// `checkdate()`: a year from 1 to 32767 and a day of that month, with the Gregorian
/// leap years (P2R-292).
const PHP_CHECKDATE: &str = r"fn php_checkdate(month: i64, day: i64, year: i64) -> bool {
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => return false,
    };
    (1..=32767).contains(&year) && (1..=days).contains(&day)
}";

/// `ucwords()` with PHP's default delimiters: the first byte and each byte after a space,
/// tab, carriage return, line feed, form feed or vertical tab become ASCII capitals
/// (P2R-303).
const PHP_UCWORDS: &str = r#"fn php_ucwords(text: &str) -> String {
    let mut bytes = text.as_bytes().to_vec();
    let mut start = true;
    for byte in &mut bytes {
        if start {
            byte.make_ascii_uppercase();
        }
        start = matches!(*byte, b' ' | b'\t' | b'\r' | b'\n' | b'\x0C' | b'\x0B');
    }
    String::from_utf8(bytes).expect("only ASCII letters changed")
}"#;

/// `preg_replace_callback()` of a pattern without `u`: the callback gets the keys "0" to
/// the last group that took part in the match, with "" for a group before it that did not,
/// as PHP passes them (P2R-302).
const PHP_PREG_REPLACE_CALLBACK: &str = r#"fn php_preg_replace_callback(
    pattern: &str,
    callback: impl Fn(&IndexMap<String, String>) -> String,
    subject: &str,
) -> String {
    let regex = ::regex::bytes::Regex::new(pattern).expect("php2rust checked this pattern");
    let bytes = subject.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut last = 0;
    for groups in regex.captures_iter(bytes) {
        let whole = groups.get(0).expect("a match has group 0");
        out.extend_from_slice(&bytes[last..whole.start()]);
        let taken = (0..groups.len())
            .rev()
            .find(|&at| groups.get(at).is_some())
            .unwrap_or(0);
        let matches: IndexMap<String, String> = (0..=taken)
            .map(|at| {
                let text = groups.get(at).map_or(&b""[..], |group| group.as_bytes());
                (at.to_string(), String::from_utf8_lossy(text).into_owned())
            })
            .collect();
        out.extend_from_slice(callback(&matches).as_bytes());
        last = whole.end();
    }
    out.extend_from_slice(&bytes[last..]);
    String::from_utf8_lossy(&out).into_owned()
}"#;
