//! PHP patterns without the `u` flag as patterns of `regex::bytes` in `(?-u)` mode, from an
//! allow-list of the syntax both engines read alike (P2R-260).

use mago_syntax::cst::*;
use regex_syntax::hir::{Hir, HirKind};

use super::Converter;
use super::helpers::Helper;
use super::lists::unparenthesized;
use crate::printer::{BinOp, RExpr};
use crate::types::RustType;

impl Converter<'_> {
    /// The value of a pattern or replacement argument: a string literal, or a constant of a
    /// class or enum of the file whose value is a string literal (P2R-260, P2R-261).
    fn literal_text(&mut self, node: &Expression<'_>) -> Option<String> {
        match unparenthesized(node) {
            Expression::Literal(Literal::String(string)) => self.string_literal(string),
            Expression::Access(Access::ClassConstant(access)) => self.constant_text(access),
            _ => None,
        }
    }

    /// The pattern argument `node` converted, or `None` with a problem recorded (P2R-260).
    fn pattern_argument(&mut self, node: &Expression<'_>) -> Option<Pattern> {
        let Some(text) = self.literal_text(node) else {
            self.record_refused(node);
            self.refuse(
                node,
                "a pattern converts only as a string literal or a constant of the file whose value is one (P2R-260)",
            );
            return None;
        };
        match rust_pattern(text.as_bytes()) {
            Ok(pattern) => Some(pattern),
            Err(message) => {
                self.refuse(node, message);
                None
            }
        }
    }

    /// `preg_replace(P, R, S) ?? 'd'` as `php_preg_replace(r"..", "R", s)`. The default is
    /// dropped: PHP gives null only on a PCRE limit error (P2R-261).
    pub(super) fn preg_replace(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
        default: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let arguments = self.built_in_arguments(
            call,
            node,
            (3, 3),
            "preg_replace() converts with three arguments only (P2R-261)",
        )?;
        let pattern = self.pattern_argument(arguments[0]);
        let replacement = match self.literal_text(arguments[1]) {
            Some(text) if !text.contains(['$', '\\']) => Some(text),
            found => {
                self.record_refused(arguments[1]);
                let message = if found.is_some() {
                    "a replacement with $ or \\ is not supported, because PHP reads them as group references (P2R-261)"
                } else {
                    "a replacement converts only as a string literal or a constant of the file whose value is one (P2R-261)"
                };
                self.refuse(arguments[1], message);
                None
            }
        };
        let subject = self.typed_argument(
            arguments[2],
            &RustType::String,
            "preg_replace() converts on a string subject only (P2R-261)",
        );
        // A default that is not a literal could hold the only read of a variable.
        if !matches!(
            unparenthesized(default),
            Expression::Literal(Literal::String(_))
        ) {
            self.record_refused(default);
            self.refuse(
                default,
                "the default of ?? after preg_replace() must be a string literal, which the conversion drops (P2R-261)",
            );
            return None;
        }
        let (pattern, replacement, subject) = (pattern?, replacement?, subject?);
        if pattern.empty {
            self.refuse(
                arguments[0],
                "a pattern that can match the empty string is not supported in preg_replace(), because PCRE also matches empty right after a match where regex does not (P2R-261)",
            );
            return None;
        }
        let subject = self.borrowed(subject, &RustType::String);
        let arguments = vec![raw(&pattern.text), RExpr::Str(replacement), subject];
        Some((
            self.helper_call(Helper::PhpPregReplace, arguments),
            RustType::String,
        ))
    }

    /// `preg_match(P, S)` read as a bool, as `php_preg_match(r"..", s)` (P2R-262).
    pub(super) fn preg_match(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<RExpr> {
        let arguments = self.built_in_arguments(
            call,
            node,
            (2, 2),
            "preg_match() converts with two arguments only (P2R-262)",
        )?;
        let pattern = self.pattern_argument(arguments[0]);
        let subject = self.typed_argument(
            arguments[1],
            &RustType::String,
            "preg_match() converts on a string subject only (P2R-262)",
        );
        let (pattern, subject) = (pattern?, subject?);
        let subject = self.borrowed(subject, &RustType::String);
        Some(self.helper_call(Helper::PhpPregMatch, vec![raw(&pattern.text), subject]))
    }

    /// `preg_match(..) === 1`, `!== 0`, `=== 0` and `!== 1`, with the number on either side,
    /// or `None` when the comparison is not one of these (P2R-262).
    pub(super) fn preg_match_comparison(
        &mut self,
        binary: &Binary<'_>,
        op: BinOp,
    ) -> Option<Option<(RExpr, RustType)>> {
        let (call, call_node, number) = match (
            self.preg_match_call(binary.lhs),
            self.preg_match_call(binary.rhs),
        ) {
            (Some(call), None) => (call, binary.lhs, binary.rhs),
            (None, Some(call)) => (call, binary.rhs, binary.lhs),
            _ => return None,
        };
        let one = match unparenthesized(number) {
            Expression::Literal(Literal::Integer(LiteralInteger { value: Some(0), .. })) => false,
            Expression::Literal(Literal::Integer(LiteralInteger { value: Some(1), .. })) => true,
            _ => return None,
        };
        let matched = self.preg_match(call, call_node);
        Some(matched.map(|matched| {
            let expr = if (op == BinOp::Eq) == one {
                matched
            } else {
                RExpr::Not(Box::new(matched))
            };
            (expr, RustType::Bool)
        }))
    }

    /// The `preg_match()` call that `expression` is, when it is one (P2R-262).
    pub(super) fn preg_match_call<'e, 'a>(
        &self,
        expression: &'e Expression<'a>,
    ) -> Option<&'e FunctionCall<'a>> {
        match unparenthesized(expression) {
            Expression::Call(Call::Function(call)) if self.calls_built_in(call, b"preg_match") => {
                Some(call)
            }
            _ => None,
        }
    }
}

/// The Rust pattern as a raw string literal, with as many `#` as its quotes need.
fn raw(text: &str) -> RExpr {
    let mut hashes = String::new();
    while text.contains(&format!("\"{hashes}")) {
        hashes.push('#');
    }
    RExpr::Path(format!("r{hashes}\"{text}\"{hashes}"))
}

/// The POSIX classes PCRE and `regex` both read alike inside a class (P2R-260).
const POSIX: [&str; 14] = [
    "alnum", "alpha", "ascii", "blank", "cntrl", "digit", "graph", "lower", "print", "punct",
    "space", "upper", "word", "xdigit",
];

/// A converted pattern: the Rust pattern text, and whether it can match the empty string.
pub(super) struct Pattern {
    pub(super) text: String,
    pub(super) empty: bool,
}

/// What the last part of the body was, for the quantifier rules (P2R-260).
#[derive(Clone, Copy, PartialEq)]
enum Last {
    /// The start of the body or of a group, or an alternation.
    Open,
    /// A literal, `.`, an escape class or a class.
    Atom,
    /// A group, which takes `*`, `+` and `?` but no count.
    Group,
    /// `^`, `$`, `\b`, `\B`, `\A` or `\z`.
    Assertion,
    /// A quantifier, which a `?` may make lazy.
    Quantifier,
    /// A lazy quantifier.
    Lazy,
}

/// The Rust pattern of the PHP pattern `php`, or the problem that stops it (P2R-260).
pub(super) fn rust_pattern(php: &[u8]) -> Result<Pattern, String> {
    if !php.iter().all(|byte| (0x20..=0x7e).contains(byte)) {
        return Err(problem("only printable ASCII bytes"));
    }
    // A two-letter class of 4 bytes compiles to 33 in PCRE, which rejects a pattern over
    // 64 KiB: 1990 copies of `[ab]` fail in PHP.
    if php.len() > 4096 {
        return Err(problem(
            "4096 bytes at most, because PCRE rejects a pattern whose compiled form passes 64 KiB",
        ));
    }
    let (body, flags) = split(php)?;
    if !flags.iter().all(|flag| b"ims".contains(flag)) {
        return Err(problem("the flags i, m and s only"));
    }
    let mut prefix = String::from("(?-u)");
    for (flag, text) in [(b'i', "(?i)"), (b'm', "(?m)"), (b's', "(?s)")] {
        if flags.contains(&flag) {
            prefix.push_str(text);
        }
    }
    let text = format!("{prefix}{}", translate(body, flags.contains(&b'm'))?);
    if regex::bytes::Regex::new(&text).is_err() {
        return Err("this pattern does not compile in Rust's regex crate (P2R-260)".to_owned());
    }
    let hir = regex_syntax::ParserBuilder::new()
        .utf8(false)
        .build()
        .parse(&text)
        .map_err(|_| "this pattern does not compile in Rust's regex crate (P2R-260)".to_owned())?;
    if empty_repeat(&hir) {
        return Err(
            "a repeat other than ? of a part that can match the empty string is not supported, because PCRE ends the loop where regex goes on (P2R-260)"
                .to_owned(),
        );
    }
    let empty = hir.properties().minimum_len() == Some(0);
    Ok(Pattern { text, empty })
}

/// The problem of a pattern outside the allow-list, naming what it may hold.
fn problem(what: &str) -> String {
    format!("a pattern converts with {what} (P2R-260)")
}

/// The body and the flags of `php`, between and after its delimiters, which nest when they
/// are brackets, as PHP counts them.
fn split(php: &[u8]) -> Result<(&[u8], &[u8]), String> {
    let Some(&open) = php.first() else {
        return Err(problem("a delimiter"));
    };
    if open.is_ascii_alphanumeric() || open == b'\\' || open == b' ' {
        return Err(problem(
            "a delimiter other than a letter, a digit, a backslash or a space",
        ));
    }
    let close = match open {
        b'(' => b')',
        b'{' => b'}',
        b'[' => b']',
        b'<' => b'>',
        other => other,
    };
    let mut depth = 1;
    let mut at = 1;
    while at < php.len() {
        match php[at] {
            b'\\' => at += 1,
            byte if byte == close => {
                depth -= 1;
                if depth == 0 {
                    return Ok((&php[1..at], &php[at + 1..]));
                }
            }
            byte if byte == open => depth += 1,
            _ => {}
        }
        at += 1;
    }
    Err(problem("a closing delimiter"))
}

/// The Rust body of the PCRE `body`, checked against the allow-list.
fn translate(body: &[u8], multiline: bool) -> Result<String, String> {
    let mut out = String::new();
    let mut last = Last::Open;
    let mut at = 0;
    while at < body.len() {
        let byte = body[at];
        match byte {
            b'\\' => {
                let (text, kind, used) = escape(&body[at..], false)?;
                out.push_str(&text);
                last = kind;
                at += used;
                continue;
            }
            b'[' => {
                let used = class(&body[at..], &mut out)?;
                last = Last::Atom;
                at += used;
                continue;
            }
            b'(' => {
                if body.get(at + 1) == Some(&b'?') {
                    if body.get(at + 2) != Some(&b':') {
                        return Err(problem("groups ( ) and (?: ) only"));
                    }
                    out.push_str("(?:");
                    at += 3;
                } else {
                    out.push('(');
                    at += 1;
                }
                last = Last::Open;
                continue;
            }
            b')' => last = Last::Group,
            b'|' => last = Last::Open,
            b'*' | b'+' | b'?' => {
                last = match (last, byte) {
                    (Last::Atom | Last::Group, _) => Last::Quantifier,
                    (Last::Quantifier, b'?') => Last::Lazy,
                    (Last::Quantifier, b'+') => {
                        return Err(problem("no possessive quantifier"));
                    }
                    _ => {
                        return Err(problem(
                            "a quantifier only after a literal, ., a class or a group",
                        ));
                    }
                };
            }
            b'{' => {
                let used = braces(&body[at..])?;
                // PCRE copies the group once per count and rejects the pattern past its
                // size limit, which depends on the group's compiled size.
                if last == Last::Group {
                    return Err(problem(
                        "a count {n}, {n,} or {n,m} after a literal, ., an escape class or a class only, not after a group",
                    ));
                }
                if last != Last::Atom {
                    return Err(problem(
                        "a quantifier only after a literal, ., a class or a group",
                    ));
                }
                out.push_str(std::str::from_utf8(&body[at..at + used]).unwrap_or_default());
                last = Last::Quantifier;
                at += used;
                continue;
            }
            b'^' if multiline => {
                return Err(
                    "^ with the m flag is not supported, because PCRE's multiline ^ does not match after a newline that ends the subject (P2R-260)"
                        .to_owned(),
                );
            }
            b'$' if !multiline => {
                return Err(
                    "$ without the m flag is not supported, because PCRE's $ also matches before a final newline (P2R-260)"
                        .to_owned(),
                );
            }
            b'^' | b'$' => last = Last::Assertion,
            _ => last = Last::Atom,
        }
        out.push(char::from(byte));
        at += 1;
    }
    Ok(out)
}

/// The Rust text of the escape at the start of `rest`, what it is, and how many bytes it
/// takes. Inside a class an escape is never an assertion.
fn escape(rest: &[u8], in_class: bool) -> Result<(String, Last, usize), String> {
    let Some(&next) = rest.get(1) else {
        return Err(problem("no backslash at its end"));
    };
    let plain = |text: String, used| Ok((text, Last::Atom, used));
    match next {
        b'd' | b'D' | b'w' | b'W' | b's' | b'S' | b't' | b'n' | b'r' | b'f' => {
            plain(format!("\\{}", char::from(next)), 2)
        }
        b'b' | b'B' | b'A' | b'z' if !in_class => {
            if next == b'b' && rest.get(2) == Some(&b'{') {
                return Err(problem("no \\b{"));
            }
            Ok((format!("\\{}", char::from(next)), Last::Assertion, 2))
        }
        b'x' if rest.get(2) == Some(&b'{') => {
            let end = rest
                .iter()
                .position(|&byte| byte == b'}')
                .ok_or_else(|| problem("a closed \\x{..}"))?;
            let digits = std::str::from_utf8(&rest[3..end]).unwrap_or_default();
            match u32::from_str_radix(digits, 16) {
                Ok(value) if value <= 0x7f => plain(format!("\\x{{{digits}}}"), end + 1),
                _ => Err(problem("\\x{..} up to 7F only")),
            }
        }
        b'x' => {
            let digits = rest.get(2..4).unwrap_or_default();
            if digits.len() == 2 && digits.iter().all(u8::is_ascii_hexdigit) {
                plain(
                    format!("\\x{}", std::str::from_utf8(digits).unwrap_or_default()),
                    4,
                )
            } else {
                Err(problem("\\x and two hex digits"))
            }
        }
        // `regex` reads `\<` and `\>` as word boundaries.
        b'<' | b'>' => plain(char::from(next).to_string(), 2),
        punctuation if punctuation.is_ascii_punctuation() => {
            plain(format!("\\{}", char::from(punctuation)), 2)
        }
        _ => Err(problem(
            "escapes of punctuation, \\d \\D \\w \\W \\s \\S \\b \\B \\A \\z \\t \\n \\r \\f and \\x only",
        )),
    }
}

/// Copies the class at the start of `rest` into `out`, checked against the allow-list.
/// Gives how many bytes it takes.
fn class(rest: &[u8], out: &mut String) -> Result<usize, String> {
    if posix_syntax(rest) {
        return Err(problem(
            "no [:name:], [.x.] or [=x=] outside a class, which PCRE rejects",
        ));
    }
    let mut at = 1;
    if rest.get(at) == Some(&b'^') {
        at += 1;
    }
    let start = at;
    let mut text = String::from("[");
    text.push_str(std::str::from_utf8(&rest[1..at]).unwrap_or_default());
    // A `]` first in the class is a literal.
    if rest.get(at) == Some(&b']') {
        text.push_str("\\]");
        at += 1;
    }
    while at < rest.len() {
        let byte = rest[at];
        match byte {
            b']' => {
                text.push(']');
                out.push_str(&text);
                return Ok(at + 1);
            }
            b'\\' => {
                let (escaped, _, used) = escape(&rest[at..], true)?;
                text.push_str(&escaped);
                at += used;
                continue;
            }
            b'[' => {
                let used = posix(&rest[at..])?;
                if rest.get(at + used) == Some(&b'-') && rest.get(at + used + 1) != Some(&b']') {
                    return Err(problem("no range from a POSIX class"));
                }
                if text.ends_with('-') && at > start + 1 {
                    return Err(problem("no range to a POSIX class"));
                }
                text.push_str(std::str::from_utf8(&rest[at..at + used]).unwrap_or_default());
                at += used;
                continue;
            }
            b'&' | b'-' | b'~' if rest.get(at + 1) == Some(&byte) => {
                return Err(problem(
                    "no && -- or ~~ in a class, which regex reads as set operators",
                ));
            }
            _ => text.push(char::from(byte)),
        }
        at += 1;
    }
    Err(problem("a closed class"))
}

/// PCRE's `check_posix_syntax` for the `[` at the start of `rest`: true when a `:`, `.` or
/// `=` follows it and comes back before `]` with no `]` or `[` of the same kind before
/// that. PCRE rejects such a form outside a class.
fn posix_syntax(rest: &[u8]) -> bool {
    let Some(&terminator) = rest
        .get(1)
        .filter(|&&byte| matches!(byte, b':' | b'.' | b'='))
    else {
        return false;
    };
    let mut at = 2;
    while at + 1 < rest.len() {
        match (rest[at], rest[at + 1]) {
            (b'\\', b']' | b'\\') => at += 1,
            (b'[', next) if next == terminator => return false,
            (b']', _) => return false,
            (byte, b']') if byte == terminator => return true,
            _ => {}
        }
        at += 1;
    }
    false
}

/// The length of the POSIX class `[:name:]` or `[:^name:]` at the start of `rest`, when its
/// name is listed (P2R-260).
fn posix(rest: &[u8]) -> Result<usize, String> {
    let inner = rest
        .strip_prefix(b"[:")
        .ok_or_else(|| problem("[ in a class only as a POSIX class"))?;
    let inner = inner.strip_prefix(b"^").unwrap_or(inner);
    let end = inner
        .windows(2)
        .position(|pair| pair == b":]")
        .ok_or_else(|| problem("[ in a class only as a POSIX class"))?;
    let name = std::str::from_utf8(&inner[..end]).unwrap_or_default();
    if !POSIX.contains(&name) {
        return Err(problem("the listed POSIX classes only"));
    }
    Ok(rest.len() - inner.len() + end + 2)
}

/// The length of the quantifier `{n}`, `{n,}` or `{n,m}` at the start of `rest`, with counts
/// up to 65535, as PCRE allows.
fn braces(rest: &[u8]) -> Result<usize, String> {
    let end = rest
        .iter()
        .position(|&byte| byte == b'}')
        .ok_or_else(|| problem("a brace only as a quantifier {n}, {n,} or {n,m}"))?;
    let inner = std::str::from_utf8(&rest[1..end]).unwrap_or_default();
    let (low, high) = inner.split_once(',').unwrap_or((inner, inner));
    let count = |text: &str| text.parse::<u32>().ok().filter(|&count| count <= 65535);
    let valid = count(low).is_some() && (high.is_empty() || count(high).is_some());
    if !valid {
        return Err(problem(
            "a brace only as a quantifier {n}, {n,} or {n,m} with counts up to 65535",
        ));
    }
    Ok(end + 1)
}

/// True when the pattern holds a repeat other than `?` whose part can match the empty
/// string: PCRE ends such a loop when the part matches empty, where `regex` tries the next
/// alternative (P2R-260).
fn empty_repeat(hir: &Hir) -> bool {
    if let HirKind::Repetition(repetition) = hir.kind()
        && repetition.max.is_none_or(|max| max > 1)
        && repetition.sub.properties().minimum_len() == Some(0)
    {
        return true;
    }
    hir.kind().subs().iter().any(empty_repeat)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(php: &str) -> Result<String, String> {
        rust_pattern(php.as_bytes()).map(|pattern| pattern.text)
    }

    #[test]
    fn the_targets_convert() {
        assert_eq!(text(r"#\D#").as_deref(), Ok(r"(?-u)\D"));
        assert_eq!(text("# {2,}#").as_deref(), Ok("(?-u) {2,}"));
        assert_eq!(text(r"<a\>b>i").as_deref(), Ok("(?-u)(?i)a>b"));
        assert_eq!(text("{a{2}}").as_deref(), Ok("(?-u)a{2}"));
        // Flags come once each, in one order; PCRE reads `[:]` and `[^:ab:]` as plain
        // classes; a group takes `+`.
        assert_eq!(text("/a/sii").as_deref(), Ok("(?-u)(?i)(?s)a"));
        assert_eq!(text("/[:]/").as_deref(), Ok("(?-u)[:]"));
        assert_eq!(text("/[^:ab:]/").as_deref(), Ok("(?-u)[^:ab:]"));
        assert_eq!(text("/(?:ab)+/").as_deref(), Ok("(?-u)(?:ab)+"));
    }

    #[test]
    fn forms_outside_the_list_stop() {
        for php in [
            r"/\v/",
            r"/\x{e9}/",
            r"/a\b{end}/",
            "/[a[b]c]/",
            "/[a&&b]/",
            "/[+--]/",
            "/^a/m",
            "/x(?:a?|b)*/",
            "/a**/",
            "/^*a/",
            "/[[:foo:]]/",
            "/[:alpha:]/",
            "/a$/",
            "<a<>",
            "/a/u",
            "/(?=a)/",
            r"/\1/",
            "/a{70000}/",
            "/[[:alpha:]-z]/",
            "/[.a.]/",
            "/[=a=]/",
            "/(?:a){65535}/",
            "/(a){2,}/",
        ] {
            assert!(text(php).is_err(), "{php} converted");
        }
        assert!(text(&format!("/{}/", "[ab]".repeat(1990))).is_err());
        assert!(text(&format!("/{}/", "[ab]".repeat(1000))).is_ok());
    }

    #[test]
    fn a_pattern_that_can_match_empty_says_so() {
        assert!(rust_pattern(b"#a*#").is_ok_and(|pattern| pattern.empty));
        assert!(rust_pattern(br"#\D#").is_ok_and(|pattern| !pattern.empty));
    }
}
