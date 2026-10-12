//! Strings: `.`, interpolation, `strlen`, and the owned and borrowed forms Rust needs
//! (P2R-20, P2R-24, P2R-26, P2R-45, P2R-48).
//!
//! A joined string becomes one `format!` whose literal text has its braces doubled and
//! whose values are positional `{}` arguments (P2R-45). A string literal and a string
//! parameter are a `&str` in Rust (P2R-24); every other string value is an owned
//! `String`. `owned` and `borrowed` convert a value to the form a place needs.

use mago_span::Span;
use mago_syntax::cst::*;

use super::Converter;
use crate::printer::RExpr;
use crate::types::{self, RustType};

/// A part of a joined string.
enum Piece {
    Text(String),
    /// A value printed with `{}`, with its type.
    Value(RExpr, RustType),
}

impl Converter<'_> {
    /// A chain of `.`, which becomes one joined string (P2R-45).
    pub(super) fn concatenation(&mut self, binary: &Binary<'_>) -> Option<(RExpr, RustType)> {
        let mut pieces = Vec::new();
        let left = self.concatenated(binary.lhs, &mut pieces);
        let right = self.concatenated(binary.rhs, &mut pieces);
        (left && right).then(|| (joined(pieces), RustType::String))
    }

    /// A value as one joined string: an interpolated string, or the right side of `.=`
    /// (P2R-45).
    pub(super) fn joined_string(&mut self, value: &Expression<'_>) -> Option<(RExpr, RustType)> {
        let mut pieces = Vec::new();
        self.concatenated(value, &mut pieces)
            .then(|| (joined(pieces), RustType::String))
    }

    /// `strlen()`: the length in bytes as `i64`, which Rust's `len` gives (P2R-48).
    pub(super) fn strlen(
        &mut self,
        call: &FunctionCall<'_>,
        node: &Expression<'_>,
    ) -> Option<(RExpr, RustType)> {
        let [Argument::Positional(argument)] = call.argument_list.arguments.as_slice() else {
            self.record_refused(node);
            self.refuse(node, "strlen takes one positional argument (P2R-48)");
            return None;
        };
        let (expr, _) = self.expression(argument.value)?;
        Some((RExpr::Len(Box::new(expr)), RustType::I64))
    }

    /// A value where Rust needs an owned `String` or `Vec`: an assignment or a return. A
    /// literal and a parameter are copied into one (P2R-26). A variable read moves; the
    /// caller decides whether it needs a copy.
    pub(super) fn owned(&self, expr: RExpr, ty: &RustType) -> RExpr {
        match expr {
            RExpr::Str(_) => RExpr::Call("String::from".into(), vec![expr]),
            RExpr::Var(name) if *ty == RustType::String && self.variables.is_borrowed(&name) => {
                RExpr::Method(Box::new(RExpr::Var(name)), "to_string")
            }
            // A slice cannot move into a `Vec`.
            RExpr::Var(name)
                if matches!(ty, RustType::Vec(_)) && self.variables.is_borrowed(&name) =>
            {
                RExpr::Method(Box::new(RExpr::Var(name)), "to_vec")
            }
            // An `Option<&str>` or `Option<&[T]>` parameter (P2R-101).
            RExpr::Var(name)
                if matches!(ty, RustType::Option(_))
                    && !ty.is_copy()
                    && self.variables.is_borrowed(&name) =>
            {
                RExpr::MapOwned(Box::new(RExpr::Var(name)))
            }
            // Rust cannot move a string or list out of a list.
            RExpr::Index(..) if !ty.is_copy() => RExpr::Method(Box::new(expr), "clone"),
            // The `&str` that `??` gives with a borrowed default (P2R-105).
            lent if *ty == RustType::String && self.lends(&lent) => {
                RExpr::Method(Box::new(lent), "to_string")
            }
            other => other,
        }
    }

    /// A value where Rust takes a `&str` or a `&[T]`: a call argument or the right side of
    /// `.=`. An owned `Option<String>` or `Option<Vec<T>>` lends `Option<&str>` or
    /// `Option<&[T]>` (P2R-101).
    pub(super) fn borrowed(&self, expr: RExpr, ty: &RustType) -> RExpr {
        if matches!(ty, RustType::Option(_)) {
            return match expr {
                _ if ty.is_copy() => expr,
                RExpr::None => expr,
                RExpr::Var(ref name) if self.variables.is_borrowed(name) => expr,
                other => RExpr::Method(Box::new(other), "as_deref"),
            };
        }
        let is_reference = match &expr {
            RExpr::Str(_) => true,
            RExpr::Var(name) => self.variables.is_borrowed(name),
            other => self.lends(other),
        };
        if matches!(ty, RustType::String | RustType::Vec(_)) && !is_reference {
            RExpr::Ref(Box::new(expr))
        } else {
            expr
        }
    }

    /// Adds an operand of `.` to `pieces`. Nested chains and interpolated strings join
    /// into the same string. Gives `false` when a problem is recorded.
    fn concatenated(&mut self, operand: &Expression<'_>, pieces: &mut Vec<Piece>) -> bool {
        match operand {
            Expression::Parenthesized(inner) => self.concatenated(inner.expression, pieces),
            Expression::Binary(binary)
                if matches!(binary.operator, BinaryOperator::StringConcat(_)) =>
            {
                let left = self.concatenated(binary.lhs, pieces);
                let right = self.concatenated(binary.rhs, pieces);
                left && right
            }
            Expression::CompositeString(CompositeString::Interpolated(string)) => {
                self.interpolated(string, pieces)
            }
            other => self.joined_value(other, pieces),
        }
    }

    fn interpolated(&mut self, string: &InterpolatedString<'_>, pieces: &mut Vec<Piece>) -> bool {
        let mut converted = true;
        for part in string.parts.iter() {
            converted &= match part {
                part if empty(part) => true,
                StringPart::Literal(literal) => match self.part_text(literal) {
                    Some(text) => {
                        pieces.push(Piece::Text(text));
                        true
                    }
                    None => false,
                },
                StringPart::Expression(value)
                | StringPart::BracedExpression(BracedExpressionStringPart {
                    expression: value,
                    ..
                }) => {
                    if matches!(value, Expression::Variable(Variable::Direct(_))) {
                        self.joined_value(value, pieces)
                    } else {
                        self.record_refused(value);
                        self.refuse(
                            part,
                            "only $name and {$name} can be interpolated into a string (P2R-45)",
                        );
                        false
                    }
                }
            };
        }
        converted
    }

    /// Adds a value joined into a string. PHP prints an int as Rust does, but a float or
    /// a bool differently (P2R-45).
    fn joined_value(&mut self, value: &Expression<'_>, pieces: &mut Vec<Piece>) -> bool {
        // Mago's type decides before the value converts, so the problem is the same
        // whether floats and bools convert or not.
        if let Some(Ok(ty)) = self
            .artifacts
            .get_expression_type(value)
            .map(types::rust_type)
            && !matches!(ty, RustType::I64 | RustType::String)
        {
            self.record_refused(value);
            // PHP joins null as an empty string (P2R-106).
            let message = if matches!(ty, RustType::Option(_)) {
                "a value that may be null cannot be joined into a string (P2R-106)"
            } else {
                "only string and int values can be joined into a string (P2R-45)"
            };
            self.refuse(value, message);
            return false;
        }
        let Some((expr, ty)) = self.expression(value) else {
            return false;
        };
        pieces.push(match expr {
            RExpr::Str(text) => Piece::Text(text),
            RExpr::Int(value) => Piece::Text(value.to_string()),
            other => Piece::Value(other, ty),
        });
        true
    }

    /// The decoded text of a literal part of an interpolated string, checked as a string
    /// literal is (P2R-49).
    fn part_text(&mut self, literal: &LiteralStringPart<'_>) -> Option<String> {
        if crate::printer::octal_overflow(literal.raw) {
            self.refuse(
                literal,
                "an octal escape above \\377 is not supported (P2R-49)",
            );
            return None;
        }
        let Some(bytes) = literal.value else {
            self.refuse(literal, "this string escape is not supported (P2R-49)");
            return None;
        };
        match std::str::from_utf8(bytes) {
            Ok(text) => Some(text.to_owned()),
            Err(_) => {
                self.refuse(
                    literal,
                    "a string that is not valid UTF-8 is not supported (P2R-49)",
                );
                None
            }
        }
    }
}

/// True for a call of the built-in `name` by its plain name, in any letter case (P2R-32).
pub(super) fn calls(call: &FunctionCall<'_>, name: &[u8]) -> bool {
    matches!(call.function, Expression::Identifier(Identifier::Local(local))
        if local.value.eq_ignore_ascii_case(name))
}

/// The read of a string variable when a value is that read alone (`$s`, `"$s"`), so an
/// assignment of it moves the variable (P2R-26).
pub(super) fn sole_read(value: &Expression<'_>) -> Option<Span> {
    match value {
        Expression::Parenthesized(inner) => sole_read(inner.expression),
        Expression::Variable(Variable::Direct(variable)) => Some(variable.span),
        Expression::CompositeString(CompositeString::Interpolated(string)) => {
            let mut parts = string.parts.iter().filter(|part| !empty(part));
            match (parts.next(), parts.next()) {
                (
                    Some(
                        StringPart::Expression(inner)
                        | StringPart::BracedExpression(BracedExpressionStringPart {
                            expression: inner,
                            ..
                        }),
                    ),
                    None,
                ) => sole_read(inner),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Mago keeps an empty literal part next to an interpolated variable; it adds nothing.
fn empty(part: &StringPart<'_>) -> bool {
    matches!(part, StringPart::Literal(literal) if literal.raw.is_empty())
}

/// One string from its pieces: a literal when no value is joined, a lone string value
/// as it is, and otherwise a `format!` (P2R-45).
fn joined(mut pieces: Vec<Piece>) -> RExpr {
    if let [Piece::Value(_, RustType::String)] = pieces.as_slice()
        && let Some(Piece::Value(lone, _)) = pieces.pop()
    {
        return lone;
    }
    let (mut plain, mut format, mut values) = (String::new(), String::new(), Vec::new());
    for piece in pieces {
        match piece {
            Piece::Text(text) => {
                format.push_str(&text.replace('{', "{{").replace('}', "}}"));
                plain.push_str(&text);
            }
            Piece::Value(value, _) => {
                format.push_str("{}");
                values.push(value);
            }
        }
    }
    if values.is_empty() {
        RExpr::Str(plain)
    } else {
        RExpr::Format(format, values)
    }
}
