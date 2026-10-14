//! Rust text: string literals, format strings and expressions on one line, which
//! rustfmt then lays out (P2R-45, P2R-49, P2R-190).

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
            // rustc's text_direction_codepoint_in_literal rejects these unescaped.
            c if c.is_control()
                || matches!(c, '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}') =>
            {
                out.push_str(&format!("\\u{{{:x}}}", u32::from(c)))
            }
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

/// Stands after a string or list variable that a statement moves, for its `.clone()` when
/// the variable is read again (P2R-26). The mark is this character, six hex digits and
/// this character, resolved at the end of the body.
pub const COPY: char = '\u{2}';

/// A Rust expression before layout.
#[derive(Clone, Debug)]
pub enum RExpr {
    Int(i64),
    /// The text of a string literal.
    Str(String),
    Var(String),
    Call(String, Vec<RExpr>),
    Binary(Box<RExpr>, BinOp, Box<RExpr>),
    Neg(Box<RExpr>),
    /// A variable whose value rustc works out at compile time (P2R-50).
    Known(String, i64),
    /// A joined string: the format text with its braces doubled, and one value per `{}`
    /// (P2R-45).
    Format(String, Vec<RExpr>),
    /// `&value`, where Rust takes a `&str` or a `&[T]`.
    Ref(Box<RExpr>),
    /// `&mut value`, where a parameter takes an object to change (P2R-144).
    RefMut(Box<RExpr>),
    /// `value.method()`, for the copies PHP makes (P2R-26).
    Method(Box<RExpr>, &'static str),
    /// The byte length of a string, or the length of a list (`count()`), as `i64`
    /// (P2R-48).
    Len(Box<RExpr>),
    Bool(bool),
    Not(Box<RExpr>),
    /// A list literal: `vec![a, b]`, or `Vec::new()` when empty (P2R-47).
    List(Vec<RExpr>),
    /// A list read by index (P2R-47).
    Index(Box<RExpr>, Box<RExpr>),
    /// A float literal, written by value (P2R-49).
    Float(f64),
    /// `value as f64`: an int where a float is declared (P2R-25).
    ToFloat(Box<RExpr>),
    /// `value as usize`: the count of `str_repeat()` (P2R-184).
    ToUsize(Box<RExpr>),
    /// The literal `null` (P2R-102).
    None,
    /// A value where an `Option` is needed (P2R-102).
    Some(Box<RExpr>),
    /// `value.map(ToOwned::to_owned)`: the owned copy of an `Option<&str>` or
    /// `Option<&[T]>` parameter (P2R-101).
    MapOwned(Box<RExpr>),
    /// A path such as an enum case, `Suit::Hearts` (P2R-112).
    Path(String),
    /// `a ?? b`: `option.unwrap_or(default)`, or `option.unwrap_or_else(|| default)` when
    /// the default must run only when the option is `None` (P2R-105).
    Unwrap {
        option: Box<RExpr>,
        default: Box<RExpr>,
        lazy: bool,
    },
    /// `object.field`, a property read (P2R-134).
    Field(Box<RExpr>, String),
    /// `object.method(arguments)`, a method call (P2R-135).
    MethodCall(Box<RExpr>, String, Vec<RExpr>),
    /// `[a, b].contains(needle)`: `in_array()` on a list literal (P2R-163).
    Contains(Vec<RExpr>, Box<RExpr>),
    /// `if condition { then } else { otherwise }`: `?:` (P2R-180).
    If(Box<RExpr>, Box<RExpr>, Box<RExpr>),
    /// `let Some(v) = v`, the condition of an `if` that binds a value checked against null
    /// (P2R-205).
    Let(String),
    /// `S { field: value }`, a struct literal of an array shape, fields in PHP's order
    /// (P2R-202).
    Struct(String, Vec<(String, RExpr)>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Rem,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
    /// `/`, for `intdiv` and `fdiv` only (P2R-48).
    Div,
    /// `<<` and `>>` by a literal count from 0 to 63 (P2R-167).
    Shl,
    Shr,
}

impl BinOp {
    fn text(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Rem => "%",
            Self::Eq => "==",
            Self::Ne => "!=",
            Self::Lt => "<",
            Self::Gt => ">",
            Self::Le => "<=",
            Self::Ge => ">=",
            Self::And => "&&",
            Self::Or => "||",
            Self::Div => "/",
            Self::Shl => "<<",
            Self::Shr => ">>",
        }
    }

    /// Rust precedence, which matches PHP for these operators: `*`, `/` and `%` bind tighter
    /// than `+` and `-`, then `<<` and `>>`, then the comparisons, then `&&`, then `||`.
    /// Above them come `as` at [`CAST`], unary operators at [`UNARY`] and method calls and
    /// indexing at [`POSTFIX`].
    fn precedence(self) -> u8 {
        match self {
            Self::Mul | Self::Rem | Self::Div => 6,
            Self::Add | Self::Sub => 5,
            Self::Shl | Self::Shr => 4,
            Self::Eq | Self::Ne | Self::Lt | Self::Gt | Self::Le | Self::Ge => 3,
            Self::And => 2,
            Self::Or => 1,
        }
    }

    /// True for `==`, `!=`, `<`, `>`, `<=` and `>=`.
    pub fn compares(self) -> bool {
        self.precedence() == 3
    }

    /// True for `<<` and `>>`.
    pub fn shifts(self) -> bool {
        self.precedence() == 4
    }
}

/// `as` binds tighter than every binary operator, so `(i + 1) as usize` needs its
/// parentheses and `n * a.len() as i64` needs none.
const CAST: u8 = 7;
/// Unary minus and `!` bind tighter than every binary operator and `as`.
const UNARY: u8 = 8;
/// A method call or an index binds tighter than everything else.
const POSTFIX: u8 = 9;

/// Writes `expr` on one line with only the parentheses Rust needs (P2R-43); rustfmt lays
/// it out (P2R-190). With `anchor`, an expression made only of literals gets `_i64` on its
/// first literal, so Rust never falls back to `i32` (P2R-49).
pub fn expr(expr: &RExpr, anchor: bool) -> String {
    render(expr, anchor, 0)
}

fn render(expr: &RExpr, anchor: bool, min_precedence: u8) -> String {
    match expr {
        RExpr::Int(value) if anchor => format!("{value}_i64"),
        RExpr::Int(value) => value.to_string(),
        RExpr::Str(text) => quote(text),
        RExpr::Var(name) => name.clone(),
        RExpr::Call(name, arguments) => format!("{name}({})", call_arguments(arguments)),
        RExpr::Field(object, field) => format!("{}.{field}", render(object, false, POSTFIX)),
        RExpr::MethodCall(object, method, arguments) => format!(
            "{}.{method}({})",
            render(object, false, POSTFIX),
            call_arguments(arguments)
        ),
        // A method call binds tighter than a minus: `(-5_i64).to_string()` (P2R-166).
        RExpr::Neg(_) if UNARY < min_precedence => format!("({})", render(expr, anchor, 0)),
        // `--x` trips rustc's double_negations lint, so the inner minus gets parentheses.
        RExpr::Neg(inner) if matches!(**inner, RExpr::Neg(_)) => {
            format!("-({})", render(inner, anchor, 0))
        }
        RExpr::Neg(inner) => format!("-{}", render(inner, anchor, UNARY)),
        // Rust comparisons do not chain, so an operand that is a comparison gets
        // parentheses. Both operands share one type, which literals alone would make i32.
        RExpr::Binary(left, op, right) if op.compares() => {
            let precedence = op.precedence();
            let anchor = !anchored(left) && !anchored(right);
            let mut left = render(left, anchor, precedence + 1);
            // rustc reads `as i64 <` as the start of generic arguments, so a left operand
            // that ends in a cast gets parentheses.
            if *op == BinOp::Lt && (left.ends_with(" as i64") || left.ends_with(" as f64")) {
                left = format!("({left})");
            }
            let text = format!(
                "{left} {} {}",
                op.text(),
                render(right, false, precedence + 1)
            );
            if precedence < min_precedence {
                format!("({text})")
            } else {
                text
            }
        }
        RExpr::Binary(left, op, right) => {
            let precedence = op.precedence();
            let mut left = render(left, anchor && !anchored(expr), precedence);
            // As for `<`, rustc reads `as i64 <<` as the start of generic arguments.
            if *op == BinOp::Shl && (left.ends_with(" as i64") || left.ends_with(" as f64")) {
                left = format!("({left})");
            }
            let right = render(right, false, precedence + 1);
            let text = format!("{left} {} {right}", op.text());
            if precedence < min_precedence {
                format!("({text})")
            } else {
                text
            }
        }
        RExpr::Known(name, _) => name.clone(),
        RExpr::Format(text, values) => format!("format!({})", format_arguments(text, values)),
        // The parameter type fixes the element type, so a list literal needs no `vec!`.
        RExpr::Ref(inner) => match &**inner {
            RExpr::List(items) => format!("&[{}]", elements(items, false)),
            other => format!("&{}", render(other, false, UNARY)),
        },
        RExpr::RefMut(inner) => match &**inner {
            // The `[]` default of a `&mut [C]` parameter (P2R-168).
            RExpr::List(items) => format!("&mut [{}]", elements(items, false)),
            other => format!("&mut {}", render(other, false, UNARY)),
        },
        // The receiver fixes no type, so one made only of literals gets `_i64` (P2R-166).
        RExpr::Method(inner, method) => {
            format!("{}.{method}()", render(inner, !anchored(inner), POSTFIX))
        }
        RExpr::Len(inner) => {
            // A length fixes no element type, so a list of literals gets `_i64` (P2R-49).
            let text = format!("{}.len() as i64", render(inner, true, POSTFIX));
            if CAST < min_precedence {
                format!("({text})")
            } else {
                text
            }
        }
        RExpr::Bool(value) => value.to_string(),
        RExpr::Not(inner) => format!("!{}", render(inner, false, UNARY)),
        RExpr::List(items) if items.is_empty() => "Vec::new()".to_owned(),
        RExpr::List(items) => format!("vec![{}]", elements(items, anchor && !anchored(expr))),
        RExpr::Index(list, index) => {
            let list = render(list, anchor, POSTFIX);
            match **index {
                // Rust reads a literal index as `usize` without a conversion.
                RExpr::Int(value) if value >= 0 => format!("{list}[{value}]"),
                _ => format!("{list}[{} as usize]", render(index, true, CAST)),
            }
        }
        // Rust has no literal for infinity. Debug gives the shortest text that reads back
        // as the same value, and always a float literal (`1.0`, `1e300`).
        RExpr::Float(value) if *value == f64::INFINITY => "f64::INFINITY".into(),
        RExpr::Float(value) => format!("{value:?}"),
        // The int inside is anchored, or literals alone would be i32.
        RExpr::ToFloat(inner) => {
            let text = format!("{} as f64", render(inner, true, CAST));
            if CAST < min_precedence {
                format!("({text})")
            } else {
                text
            }
        }
        RExpr::ToUsize(inner) => {
            let text = format!("{} as usize", render(inner, true, CAST));
            if CAST < min_precedence {
                format!("({text})")
            } else {
                text
            }
        }
        RExpr::None => "None".to_owned(),
        RExpr::Some(inner) => format!("Some({})", render(inner, anchor, 0)),
        RExpr::Path(path) => path.clone(),
        // The elements fix no type, so a needle made only of literals makes the first one
        // `_i64` (P2R-49).
        RExpr::Contains(items, needle) => {
            let anchor = matches!(&**needle, RExpr::Ref(inner) if !anchored(inner));
            format!(
                "[{}].contains({})",
                elements(items, anchor),
                render(needle, false, 0)
            )
        }
        RExpr::MapOwned(inner) => {
            format!("{}.map(ToOwned::to_owned)", render(inner, false, POSTFIX))
        }
        RExpr::Unwrap {
            option,
            default,
            lazy,
        } => {
            let value = render(default, false, 0);
            let (method, argument) = if *lazy {
                ("unwrap_or_else", format!("|| {value}"))
            } else {
                ("unwrap_or", value)
            };
            format!("{}.{method}({argument})", render(option, false, POSTFIX))
        }
        RExpr::If(condition, then, otherwise) => {
            let text = format!(
                "if {} {{ {} }} else {{ {} }}",
                render(condition, false, 0),
                render(then, anchor && !anchored(expr), 0),
                render(otherwise, false, 0)
            );
            if min_precedence > 0 {
                format!("({text})")
            } else {
                text
            }
        }
        RExpr::Let(pattern) => pattern.clone(),
        RExpr::Struct(name, fields) => {
            let parts: Vec<String> = fields
                .iter()
                .map(|(field, value)| format!("{field}: {}", render(value, false, 0)))
                .collect();
            format!("{name} {{ {} }}", parts.join(", "))
        }
    }
}

/// The arguments of a call or a method call.
fn call_arguments(arguments: &[RExpr]) -> String {
    let parts: Vec<String> = arguments.iter().map(|a| render(a, false, 0)).collect();
    parts.join(", ")
}

/// The arguments of `format!` or `print!` for a joined string.
pub fn format_arguments(text: &str, values: &[RExpr]) -> String {
    let mut arguments = quote(text);
    for value in values {
        arguments.push_str(", ");
        arguments.push_str(&render(value, true, 0));
    }
    arguments
}

/// The elements of a list literal, the first that can take it with `anchor`: `None` has no
/// literal to suffix (P2R-49).
fn elements(items: &[RExpr], anchor: bool) -> String {
    let first = items.iter().position(|item| !matches!(item, RExpr::None));
    let parts: Vec<String> = items
        .iter()
        .enumerate()
        .map(|(i, item)| render(item, anchor && Some(i) == first, 0))
        .collect();
    parts.join(", ")
}

/// True when a variable or a call fixes the expression's Rust type without its literals.
pub fn anchored(expr: &RExpr) -> bool {
    match expr {
        RExpr::Int(_) | RExpr::Str(_) => false,
        RExpr::Var(_) | RExpr::Call(..) => true,
        RExpr::Neg(inner) => anchored(inner),
        // A comparison or a logic operator always gives a bool.
        RExpr::Binary(left, op, right) => {
            op.compares()
                || matches!(op, BinOp::And | BinOp::Or)
                || anchored(left)
                || anchored(right)
        }
        RExpr::Known(..) => true,
        RExpr::Format(..) | RExpr::Ref(_) | RExpr::Method(..) | RExpr::Len(_) => true,
        RExpr::RefMut(_) => true,
        RExpr::Bool(_) | RExpr::Not(_) => true,
        RExpr::List(items) => items.iter().any(anchored),
        RExpr::Index(list, _) => anchored(list),
        // A float literal without a suffix is f64, as is a cast to it.
        RExpr::Float(_) | RExpr::ToFloat(_) | RExpr::ToUsize(_) => true,
        // `None` alone, or `Some(5)`, leaves the inner type to inference (P2R-49).
        RExpr::None => false,
        RExpr::Some(inner) => anchored(inner),
        RExpr::MapOwned(_) | RExpr::Unwrap { .. } | RExpr::Path(_) => true,
        // A field and a method have declared types.
        RExpr::Field(..) | RExpr::MethodCall(..) => true,
        RExpr::Contains(..) => true,
        RExpr::If(_, then, otherwise) => anchored(then) || anchored(otherwise),
        RExpr::Let(_) | RExpr::Struct(..) => true,
    }
}

/// The value of an int expression made only of literals and known variables. `None` when
/// it holds another variable or a call, or when it overflows or divides by zero: rustc
/// evaluates such an expression at compile time and rejects it.
pub fn literal_value(expr: &RExpr) -> Option<i64> {
    match expr {
        RExpr::Int(value) => Some(*value),
        RExpr::Neg(inner) => literal_value(inner)?.checked_neg(),
        RExpr::Binary(left, op, right) => {
            let (left, right) = (literal_value(left)?, literal_value(right)?);
            match op {
                BinOp::Add => left.checked_add(right),
                BinOp::Sub => left.checked_sub(right),
                BinOp::Mul => left.checked_mul(right),
                BinOp::Rem => left.checked_rem(right),
                BinOp::Eq
                | BinOp::Ne
                | BinOp::Lt
                | BinOp::Gt
                | BinOp::Le
                | BinOp::Ge
                | BinOp::And
                | BinOp::Or => None,
                BinOp::Div => left.checked_div(right),
                // A shift loses the bits it moves out, in rustc as in PHP (P2R-167).
                BinOp::Shl => left.checked_shl(u32::try_from(right).ok()?),
                BinOp::Shr => left.checked_shr(u32::try_from(right).ok()?),
            }
        }
        RExpr::Str(_) | RExpr::Var(_) | RExpr::Call(..) => None,
        RExpr::Known(_, value) => Some(*value),
        RExpr::Format(..) | RExpr::Ref(_) | RExpr::Method(..) | RExpr::Len(_) => None,
        RExpr::RefMut(_) => None,
        RExpr::Bool(_) | RExpr::Not(_) => None,
        RExpr::List(_) | RExpr::Index(..) => None,
        RExpr::Float(_) | RExpr::ToFloat(_) | RExpr::ToUsize(_) => None,
        RExpr::None | RExpr::Some(_) | RExpr::MapOwned(_) | RExpr::Unwrap { .. } => None,
        RExpr::Path(_) | RExpr::Field(..) | RExpr::MethodCall(..) => None,
        RExpr::Contains(..) | RExpr::If(..) | RExpr::Let(_) | RExpr::Struct(..) => None,
    }
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
        assert_eq!(
            quote("a\u{202e}b\u{2066}c\u{2069}"),
            "\"a\\u{202e}b\\u{2066}c\\u{2069}\""
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

    fn int(value: i64) -> Box<RExpr> {
        Box::new(RExpr::Int(value))
    }

    #[test]
    fn parentheses_follow_rust_precedence() {
        // 10 - (4 - 1)
        let e = RExpr::Binary(
            int(10),
            BinOp::Sub,
            Box::new(RExpr::Binary(int(4), BinOp::Sub, int(1))),
        );
        assert_eq!(expr(&e, false), "10 - (4 - 1)");
        // (1 + 2) * 3
        let e = RExpr::Binary(
            Box::new(RExpr::Binary(int(1), BinOp::Add, int(2))),
            BinOp::Mul,
            int(3),
        );
        assert_eq!(expr(&e, false), "(1 + 2) * 3");
        // 1 + 2 * 3 needs none
        let e = RExpr::Binary(
            int(1),
            BinOp::Add,
            Box::new(RExpr::Binary(int(2), BinOp::Mul, int(3))),
        );
        assert_eq!(expr(&e, false), "1 + 2 * 3");
    }

    #[test]
    fn anchor_suffixes_the_first_literal_only_without_a_variable() {
        let literals = RExpr::Binary(int(2), BinOp::Add, int(3));
        assert_eq!(expr(&literals, true), "2_i64 + 3");
        let with_var = RExpr::Binary(int(2), BinOp::Add, Box::new(RExpr::Var("n".into())));
        assert_eq!(expr(&with_var, true), "2 + n");
        assert_eq!(
            expr(&RExpr::Neg(Box::new(RExpr::Neg(int(5)))), true),
            "-(-5_i64)"
        );
    }

    #[test]
    fn literal_values_refuse_overflow_and_zero_remainder() {
        assert_eq!(
            literal_value(&RExpr::Binary(int(i64::MAX), BinOp::Add, int(1))),
            None
        );
        assert_eq!(
            literal_value(&RExpr::Binary(int(7), BinOp::Rem, int(0))),
            None
        );
        assert_eq!(literal_value(&RExpr::Neg(int(5))), Some(-5));
        assert_eq!(literal_value(&RExpr::Var("n".into())), None);
    }

    fn binary(left: RExpr, op: BinOp, right: RExpr) -> RExpr {
        RExpr::Binary(Box::new(left), op, Box::new(right))
    }

    #[test]
    fn comparisons_and_logic_follow_rust_precedence() {
        let n = || RExpr::Var("n".into());
        // (n < 2) == (2 < n): Rust comparisons do not chain.
        let e = binary(
            binary(n(), BinOp::Lt, RExpr::Int(2)),
            BinOp::Eq,
            binary(RExpr::Int(2), BinOp::Lt, n()),
        );
        assert_eq!(expr(&e, false), "(n < 2) == (2 < n)");
        // n * 2 + 1 > 3 && !(n >= 4) || true needs no parentheses but those of `!`.
        let sum = binary(
            binary(n(), BinOp::Mul, RExpr::Int(2)),
            BinOp::Add,
            RExpr::Int(1),
        );
        let not = RExpr::Not(Box::new(binary(n(), BinOp::Ge, RExpr::Int(4))));
        let and = binary(binary(sum, BinOp::Gt, RExpr::Int(3)), BinOp::And, not);
        let e = binary(and.clone(), BinOp::Or, RExpr::Bool(true));
        assert_eq!(expr(&e, false), "n * 2 + 1 > 3 && !(n >= 4) || true");
        // (a || b) && c keeps its parentheses.
        let e = binary(
            binary(RExpr::Bool(false), BinOp::Or, RExpr::Bool(true)),
            BinOp::And,
            RExpr::Var("c".into()),
        );
        assert_eq!(expr(&e, false), "(false || true) && c");
    }

    #[test]
    fn a_cast_gets_parentheses_only_where_rust_needs_them() {
        let len = || RExpr::Len(Box::new(RExpr::Var("a".into())));
        let n = || RExpr::Var("n".into());
        assert_eq!(
            expr(&binary(n(), BinOp::Mul, len()), false),
            "n * a.len() as i64"
        );
        assert_eq!(
            expr(&RExpr::Neg(Box::new(len())), false),
            "-(a.len() as i64)"
        );
        // `as i64 <` would start generic arguments; `<=` and a cast on the right do not.
        assert_eq!(
            expr(&binary(len(), BinOp::Lt, RExpr::Int(3)), false),
            "(a.len() as i64) < 3"
        );
        let product = binary(n(), BinOp::Mul, len());
        assert_eq!(
            expr(&binary(product, BinOp::Lt, RExpr::Int(3)), false),
            "(n * a.len() as i64) < 3"
        );
        assert_eq!(
            expr(&binary(len(), BinOp::Le, RExpr::Int(3)), false),
            "a.len() as i64 <= 3"
        );
        assert_eq!(
            expr(&binary(RExpr::Int(3), BinOp::Lt, len()), false),
            "3 < a.len() as i64"
        );
    }

    #[test]
    fn literal_only_comparisons_are_anchored_to_i64() {
        let e = binary(RExpr::Int(3_000_000_000), BinOp::Lt, RExpr::Int(1));
        assert_eq!(expr(&e, false), "3000000000_i64 < 1");
        let e = binary(RExpr::Int(1), BinOp::Lt, RExpr::Var("n".into()));
        assert_eq!(expr(&e, false), "1 < n");
        assert!(anchored(&e));
    }

    #[test]
    fn some_and_none_leave_the_type_to_the_context() {
        let some = RExpr::Some(int(5));
        assert_eq!(expr(&some, false), "Some(5)");
        assert!(!anchored(&some));
        assert!(!anchored(&RExpr::None));
        assert!(anchored(&RExpr::Some(Box::new(RExpr::Var("n".into())))));
        // The first element that is not `None` takes the `_i64`.
        let list = RExpr::List(vec![RExpr::None, RExpr::Some(int(7)), RExpr::Some(int(8))]);
        assert_eq!(expr(&list, true), "vec![None, Some(7_i64), Some(8)]");
    }

    #[test]
    fn floats_are_written_by_value() {
        let text = |value: f64| expr(&RExpr::Float(value), true);
        assert_eq!(text(1.0), "1.0");
        assert_eq!(text(0.0), "0.0");
        assert_eq!(text(0.1), "0.1");
        assert_eq!(text(1e300), "1e300");
        assert_eq!(text(9_223_372_036_854_775_808.0), "9.223372036854776e18");
        assert_eq!(text(f64::INFINITY), "f64::INFINITY");
    }

    #[test]
    fn casts_to_float_get_parentheses_only_where_rust_needs_them() {
        let cast = |inner: RExpr| RExpr::ToFloat(Box::new(inner));
        let n = || RExpr::Var("n".into());
        assert_eq!(expr(&cast(n()), false), "n as f64");
        let quotient = binary(RExpr::Float(1.0), BinOp::Div, cast(n()));
        assert_eq!(expr(&quotient, false), "1.0 / n as f64");
        // `1.0 / n as f64 < x` does not parse: `<` reads as generic arguments of `f64`.
        let e = binary(quotient, BinOp::Lt, RExpr::Var("x".into()));
        assert_eq!(expr(&e, false), "(1.0 / n as f64) < x");
        assert_eq!(expr(&RExpr::Neg(Box::new(cast(n()))), false), "-(n as f64)");
        let sum = cast(binary(RExpr::Int(2), BinOp::Add, RExpr::Int(3)));
        assert_eq!(expr(&sum, false), "(2_i64 + 3) as f64");
        let negated = cast(RExpr::Neg(Box::new(n())));
        assert_eq!(expr(&negated, false), "-n as f64");
    }

    #[test]
    fn shifts_bind_between_sums_and_comparisons() {
        let n = || RExpr::Var("n".into());
        let sum = binary(n(), BinOp::Add, RExpr::Int(1));
        assert_eq!(
            expr(&binary(sum, BinOp::Shl, RExpr::Int(2)), false),
            "n + 1 << 2"
        );
        let shift = binary(n(), BinOp::Shl, RExpr::Int(2));
        assert_eq!(
            expr(&binary(shift, BinOp::Mul, RExpr::Int(3)), false),
            "(n << 2) * 3"
        );
        // `as i64 <<` would start generic arguments, as `as i64 <` does.
        let len = RExpr::Len(Box::new(n()));
        assert_eq!(
            expr(&binary(len, BinOp::Shl, RExpr::Int(2)), false),
            "(n.len() as i64) << 2"
        );
        let literals = binary(RExpr::Int(1), BinOp::Shl, RExpr::Int(63));
        assert_eq!(expr(&literals, true), "1_i64 << 63");
        assert_eq!(literal_value(&literals), Some(i64::MIN));
    }

    #[test]
    fn a_method_on_a_literal_or_an_operator_gets_i64_and_parentheses() {
        let to_string = |inner| RExpr::Method(Box::new(inner), "to_string");
        assert_eq!(
            expr(&to_string(RExpr::Neg(int(5))), false),
            "(-5_i64).to_string()"
        );
        assert_eq!(
            expr(&to_string(RExpr::Int(3_000_000_000)), false),
            "3000000000_i64.to_string()"
        );
        let sum = binary(RExpr::Var("n".into()), BinOp::Add, RExpr::Int(1));
        assert_eq!(expr(&to_string(sum), false), "(n + 1).to_string()");
    }

    #[test]
    fn contains_anchors_its_elements_only_for_a_literal_needle() {
        let list = || vec![RExpr::Int(1), RExpr::Int(2)];
        let needle = |inner| Box::new(RExpr::Ref(Box::new(inner)));
        let literal = RExpr::Contains(list(), needle(RExpr::Int(2)));
        assert_eq!(expr(&literal, false), "[1_i64, 2].contains(&2)");
        let variable = RExpr::Contains(list(), needle(RExpr::Var("n".into())));
        assert_eq!(expr(&variable, false), "[1, 2].contains(&n)");
        let empty = RExpr::RefMut(Box::new(RExpr::List(Vec::new())));
        assert_eq!(expr(&empty, false), "&mut []");
    }
}
